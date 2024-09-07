use crate::{
    common::indenter::Indenter,
    turtle::{
        writer,
        writer::{io_error, options::TurtleOptions, triple_type::TurtleTripleType, utf8_error},
    },
};
use itertools::Itertools;
use rdftk_core::model::{
    graph::Graph,
    qname::QName,
    statement::{ObjectNode, ObjectNodeRef, SubjectNodeRef},
};
use rdftk_iri::{Iri, IriRef};
#[allow(unused_imports)]
use std::borrow::BorrowMut as _;
use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    io::Write,
    iter::FromIterator,
    ops::{Deref, DerefMut},
    rc::Rc,
    str::FromStr,
};

type IsNextObjectBlankNode = bool;
type IsBeingSorted = bool;
type IsLastOfSubject = bool;
type IsLastOfPredicate = bool;

#[derive(Default, Copy, Clone, Debug)]
struct TurtleCursorFlags {
    /// `is_next_object_blank` is true when the next object in the series (also)
    /// a blank node? In that case we do the formatting a bit different,
    /// showing ],[` as the separator between blank nodes.
    is_next_object_blank: IsNextObjectBlankNode,
    /// `is_being_sorted` is true when we're being called by a sorting algorithm
    /// which means that we can only produce content on one (sortable) line,
    /// avoid any line-feeds..
    is_being_sorted: IsBeingSorted,
    /// `is_last_of_subject` is true when we're working on the last triple of
    /// the current subject, in which case we have to end a line with a dot
    /// instead of a semicolon.
    is_last_of_subject: IsLastOfSubject,
    /// `is_last_of_predicate` is true when the current object is the last
    /// object in the collection of objects for the given `subject +
    /// predicate`.
    is_last_of_predicate: IsLastOfPredicate,
}

pub(crate) struct TurtleCursor<'a, W>
where
    W: Write + Sized,
{
    w: Rc<RefCell<W>>,
    graph: Rc<Ref<'a, dyn Graph>>,
    indenter: RefCell<Indenter>,
    blanks_to_write: RefCell<Vec<SubjectNodeRef>>,
    blanks_written: RefCell<Vec<SubjectNodeRef>>,
    options: TurtleOptions,
}

impl<'a, W: Write + Sized> TurtleCursor<'a, W> {
    pub(crate) fn new(
        w: Rc<RefCell<W>>,
        graph: Rc<Ref<'a, (dyn Graph + 'a)>>,
        options: TurtleOptions,
    ) -> Self {
        let indenter = RefCell::new(Indenter::default().with_width(options.indent_width()));
        let blanks_to_write = RefCell::new(
            graph
                .deref()
                .blank_node_subjects()
                .iter()
                .map(|s| {
                    let x = *s;
                    x.clone()
                })
                .collect(),
        );
        let blanks_written: RefCell<Vec<SubjectNodeRef>> = RefCell::new(Default::default());
        Self {
            w,
            graph,
            indenter,
            blanks_to_write,
            blanks_written,
            options,
        }
    }

    /// Get a copy (you clone it first) of the current Cursor but then with the
    /// given writer.
    fn new_with_writer<W2: Write + Sized>(
        w: Rc<RefCell<W>>,
        other: &'a TurtleCursor<'a, W2>,
    ) -> Self {
        Self {
            w,
            graph: other.graph.clone(),
            indenter: {
                let clone = other.indenter.borrow().clone().with_depth(0);
                RefCell::new(clone)
            },
            blanks_to_write: other.blanks_to_write.clone(),
            blanks_written: other.blanks_written.clone(),
            options: other.options.clone(),
        }
    }

    pub(crate) fn write(&self) -> rdftk_core::error::Result<()> {
        self.write_base_iri()?;
        self.write_prefixes()?;
        let flags = TurtleCursorFlags::default();
        self.write_normal_subjects(flags)?;
        // The given cursor object collects all the blank-node objects that have not
        // been written to the turtle file yet but have been referred to during
        // the call to `write_normal_subjects` above. Now process those
        // unwritten blank nodes and add them to the end of the file.
        self.write_blank_node_subjects(flags)
    }

    #[allow(unused_results)]
    fn indent(&self) -> &RefCell<Indenter> {
        self.indenter.replace_with(|old| old.indent());
        &self.indenter
    }

    #[allow(unused_results)]
    #[allow(unused)]
    fn outdent(&self) -> &RefCell<Indenter> {
        self.indenter.replace_with(|old| old.outdent());
        &self.indenter
    }

    fn new_line(&self, flags: TurtleCursorFlags) -> rdftk_core::error::Result<()> {
        if flags.is_being_sorted {
            write!(self, " ").map_err(io_error)
        } else {
            write!(self, "\n{}", self.indenter.borrow()).map_err(io_error)
        }
    }

    fn write_fmt(&self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        self.w.as_ref().borrow_mut().write_fmt(fmt)
    }

    fn wrote_blank(&self, blank: &SubjectNodeRef) {
        assert!(blank.is_blank());
        self.blanks_written.borrow_mut().push(blank.clone());
    }

    fn blanks_not_written(&self) -> HashSet<SubjectNodeRef> {
        let blanks_written = self.blanks_written.borrow();
        self.blanks_to_write
            .borrow()
            .iter()
            .filter(|subject| !blanks_written.contains(subject))
            .cloned()
            .collect()
    }

    fn sorted_subjects(&self) -> Vec<SubjectNodeRef> {
        self.graph
            .node_subjects()
            .into_iter()
            .sorted()
            .cloned()
            .collect::<Vec<SubjectNodeRef>>()
    }

    /// Write out the graph base Iri in either turtle
    /// style (as '@base ..') or SPARQL style (as 'BASE ...')
    fn write_base_iri(&self) -> rdftk_core::error::Result<()> {
        if let Some(base) = &self.options.id_base() {
            if self.options.use_sparql_style() && !self.options.use_intellij_style() {
                writeln!(self, "BASE <{}>", base.to_string().as_str()).map_err(io_error)?;
            } else {
                writeln!(self, "@base <{}> .", base.to_string().as_str()).map_err(io_error)?;
            }
        }
        if !self.options.use_intellij_style() {
            writeln!(self).map_err(io_error)?;
        }
        Ok(())
    }

    /// Write all prefix mappings, sort them by prefix to avoid
    /// random order and unnecessary changes in git
    fn write_prefixes(&self) -> rdftk_core::error::Result<()> {
        let mappings = self.graph.deref().prefix_mappings();
        let mappings = mappings.try_borrow().map_err(writer::borrow_error)?;
        for (prefix, namespace) in mappings.mappings().sorted() {
            let prefix = prefix.as_ref().map(|n| n.as_ref()).unwrap_or("");
            let mut namespace_str = namespace.to_string();
            // If we have any base Iri conversions to do for any of the namespaces, then do
            // it now:
            for (from_base, to_base) in self.options.convert_base().iter() {
                let from_base_str = from_base.to_string();
                if namespace_str.starts_with(from_base_str.as_str()) {
                    namespace_str = format!(
                        "{}{}",
                        to_base.to_string().as_str(),
                        &namespace_str[from_base_str.len()..]
                    );
                    break;
                }
            }
            if self.options.use_sparql_style() && !self.options.use_intellij_style() {
                writeln!(self, "PREFIX {prefix}: <{namespace_str}>").map_err(io_error)?;
            } else {
                writeln!(self, "@prefix {prefix}: <{namespace_str}> .").map_err(io_error)?;
            }
        }
        writeln!(self).map_err(io_error)
    }

    fn with_node_subjects_do<F>(
        &self,
        flags: TurtleCursorFlags,
        f: F,
    ) -> rdftk_core::error::Result<()>
    where
        F: Fn(&Self, &SubjectNodeRef, TurtleCursorFlags) -> rdftk_core::error::Result<()>,
    {
        for subject in self.sorted_subjects() {
            f(self, &subject, flags)?;
        }
        Ok(())
    }

    fn with_unwritten_blank_node_subjects<F>(
        &self,
        flags: TurtleCursorFlags,
        f: F,
    ) -> rdftk_core::error::Result<()>
    where
        F: Fn(&Self, SubjectNodeRef, TurtleCursorFlags) -> rdftk_core::error::Result<()>,
    {
        for subject in self.blanks_not_written().into_iter() {
            self.indenter.borrow_mut().reset_depth();
            f(self, subject.clone(), flags)?;
        }
        Ok(())
    }

    fn with_predicates_grouped<F>(
        &self,
        subject: &SubjectNodeRef,
        flags: TurtleCursorFlags,
        f: F,
    ) -> rdftk_core::error::Result<()>
    where
        F: Fn(
            &Self,
            TurtleTripleType,
            &IriRef,
            usize,
            TurtleCursorFlags,
        ) -> rdftk_core::error::Result<()>,
    {
        let all_predicates = Vec::from_iter(self.graph.predicates_for(subject));
        let mut count = 0;
        let total_number = all_predicates.len();
        let max_len = 1 + self.max_len_predicates(&all_predicates)?;

        // let max_len = all_predicates.iter().(|)
        //     .fold(std::u16::MIN, |a,b| a.max(b.borrow().));
        for (group, ref mut preds) in TurtleTripleType::group_predicates(&all_predicates) {
            preds.sort_by_cached_key(|iri| self.predicate_iri_as_string(iri).unwrap());
            for predicate in preds {
                count += 1;
                let flags = TurtleCursorFlags {
                    is_last_of_subject: count == total_number,
                    ..flags
                };
                f(self, group, predicate, max_len, flags)?;
            }
        }

        Ok(())
    }

    fn max_len_predicates(&self, predicates: &[&IriRef]) -> rdftk_core::error::Result<usize> {
        let all_predicates_as_strings = predicates
            .iter()
            .map(|iri| {
                let mut buffer = Vec::<u8>::new();
                self.compress_iri(&mut buffer, iri).map_err(io_error)?;
                String::from_utf8(buffer).map_err(utf8_error)
            })
            .collect::<Result<Vec<String>, rdftk_core::error::Error>>()?
            .iter()
            .fold(0, |a, b| a.max(b.len()));
        Ok(all_predicates_as_strings)
    }

    /// Iterate through all sorted objects of the given subject and predicate
    fn with_objects<F>(
        &self,
        subject: &SubjectNodeRef,
        predicate: &IriRef,
        flags: TurtleCursorFlags,
        f: F,
    ) -> rdftk_core::error::Result<()>
    where
        F: Fn(&Self, &ObjectNodeRef, TurtleCursorFlags) -> rdftk_core::error::Result<()>,
    {
        let mut objects = self
            .graph
            .deref()
            .objects_for(subject, predicate)
            .into_iter()
            .collect_vec();
        let is_collection_of_objects = objects.len() > 1;
        if is_collection_of_objects {
            objects.sort_by_key(|o| self.object_sort_key(o).unwrap_or_default());
        }
        let mut o_iter = objects.iter().peekable();
        while let Some(object) = o_iter.next() {
            let next_object = o_iter.peek();
            let flags = TurtleCursorFlags {
                is_next_object_blank: next_object.is_some() && next_object.unwrap().is_blank(),
                is_last_of_predicate: next_object.is_none(),
                ..flags
            };
            f(self, object, flags)?;
        }
        Ok(())
    }

    fn object_sort_key(&self, object: &ObjectNodeRef) -> rdftk_core::error::Result<String> {
        let buffer = Rc::new(RefCell::from(Vec::<u8>::new()));
        let new_cursor = TurtleCursor::new_with_writer(buffer.clone(), self);
        let flags = TurtleCursorFlags {
            is_being_sorted: true,
            ..Default::default()
        };
        new_cursor.write_object_content(object, flags)?;
        String::from_utf8(buffer.take()).map_err(utf8_error)
    }

    fn write_object_content(
        &self,
        object: &ObjectNodeRef,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        if object.is_blank() {
            if self.options.nest_blank_nodes() {
                self.write_nested_blank_node(object, flags)?;
            } else {
                write!(self, "_:{}", object.as_blank().unwrap()).map_err(io_error)?;
            }
        } else if object.is_iri() {
            self.write_iri(object.as_iri().unwrap()).map_err(io_error)?;
        } else {
            self.write_literal(object).map_err(io_error)?;
        }
        Ok(())
    }

    /// Write out a given Iri as Turtle.
    /// Compress any Iri to its "QName" given the supplied set of prefixes and
    /// their namespace Iris. If we're encountering an Iri whose prefix
    /// equals the given (optional) `convert_to_base` Iri then write it to
    /// Turtle as if it's an Iri with the default base.
    fn write_iri(&self, iri: &IriRef) -> std::io::Result<()> {
        self.compress_iri(self.w.deref().borrow_mut().deref_mut(), iri)
    }

    fn compress_iri<W2: Write + Sized>(
        &self,
        writer: &mut W2,
        iri: &IriRef,
    ) -> std::io::Result<()> {
        let mut iri_str = iri.to_string();
        if let Some(id_base) = &self.options.id_base() {
            if let Some(ref convert_to_id_base) = self.options.convert_to_id_base() {
                let target_id_base = convert_to_id_base.to_string();
                if iri_str.starts_with(target_id_base.as_str()) {
                    return write!(writer, "<{}>", &iri_str[target_id_base.len()..]);
                }
            }
            let id_base_str = id_base.to_string();
            if iri_str.starts_with(id_base_str.as_str()) {
                return write!(writer, "<{}>", &iri_str[id_base_str.len()..]);
            }
        }
        for (from_base, to_base) in self.options.convert_base().iter() {
            let from_base_str = from_base.to_string();
            if iri_str.starts_with(from_base_str.as_str()) {
                iri_str = format!(
                    "{}{}",
                    to_base.to_string().as_str(),
                    &iri_str[from_base_str.len()..]
                );
            }
        }
        let iri = IriRef::new(Iri::from_str(iri_str.as_str()).unwrap());
        match self.compress(&iri) {
            None => write!(writer, "<{iri}>"),
            Some(_qname) => write!(writer, "{_qname}"),
        }
    }

    /// Write statements, start with those where subject is an Iri,
    /// sort them by URL so that we keep a consistent result avoiding git-diff
    /// to flag certain lines as changed.
    fn write_normal_subjects(&self, flags: TurtleCursorFlags) -> rdftk_core::error::Result<()> {
        self.with_node_subjects_do(flags, |c, subject, flags| {
            c.write_sub_graph(subject, flags)?;
            writeln!(c).map_err(io_error)?;
            Ok(())
        })
    }

    /// Write statements where subject is a blank node
    fn write_blank_node_subjects(&self, flags: TurtleCursorFlags) -> rdftk_core::error::Result<()> {
        self.with_unwritten_blank_node_subjects(flags, |c, ref subject, flags| {
            c.write_sub_graph(subject, flags)?;
            Ok(())
        })
    }

    fn write_subject(
        &self,
        subject: SubjectNodeRef,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        if subject.is_blank() && self.indenter.borrow().depth() == 0 {
            if flags.is_being_sorted {
                write!(self, " _:{}", subject.as_blank().unwrap()).map_err(io_error)?;
            } else {
                write!(self, "\n_:{}", subject.as_blank().unwrap()).map_err(io_error)?;
            }
        } else if subject.is_iri() {
            self.write_iri(subject.as_iri().unwrap())
                .map_err(io_error)?;
        }
        let _ = self.indent();
        Ok(())
    }

    /// Compress an Iri into a qname, if possible.
    fn compress(&self, iri: &IriRef) -> Option<QName> {
        self.graph
            .deref()
            .prefix_mappings()
            .deref()
            .borrow()
            .compress(iri)
    }

    fn write_literal(&self, literal: &ObjectNodeRef) -> std::io::Result<()> {
        // TODO: compress data type Iris
        if let Some(literal) = literal.as_literal() {
            write!(self, "{}", literal)
        } else {
            write!(self, "ERROR: this is not a literal: {:?}", literal)
        }
    }

    fn write_predicate(
        &self,
        group: TurtleTripleType,
        predicate: &IriRef,
        max_len: usize,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        // Special treatment for `rdf:type`; show it in turtle as just "a"
        //
        if group == TurtleTripleType::Type {
            return if self.options.place_type_on_subject_line() {
                write!(self, " a ").map_err(io_error)
            } else {
                self.new_line(flags)?;
                write!(self, "{:<max_len$}", "a").map_err(io_error)
            };
        }
        // Otherwise, go to the next line and write it as a normal predicate-Iri
        //
        self.new_line(flags)?;
        let pred = self.predicate_iri_as_string(predicate)?;
        write!(self, "{:<max_len$}", pred.as_str()).map_err(io_error)
    }

    fn predicate_iri_as_string(&self, predicate: &IriRef) -> rdftk_core::error::Result<String> {
        let buffer = Rc::new(RefCell::from(Vec::<u8>::new()));
        let new_cursor = TurtleCursor::new_with_writer(buffer.clone(), self);
        new_cursor.write_iri(predicate).map_err(io_error)?;
        String::from_utf8(buffer.take()).map_err(utf8_error)
    }

    fn write_object(
        &self,
        object: &ObjectNodeRef,
        max_len: usize,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        // if flags.is_collection_of_objects && flags.{
        //     self.new_line(flags)?;
        //     write!(self, "{:max_len$}", " ").map_err(io_error)?;
        // }
        self.write_object_content(object, flags)?;
        if flags.is_last_of_predicate {
            if flags.is_last_of_subject {
                let _ = self.outdent();
            }
            if self.indenter.borrow().depth() == 0 {
                write!(self, " .").map_err(io_error)?;
                self.new_line(flags)?;
            } else if !flags.is_last_of_subject {
                write!(self, " ;").map_err(io_error)?;
            }
        } else {
            write!(self, ",").map_err(io_error)?;
            if !flags.is_next_object_blank {
                self.new_line(flags)?;
                write!(self, "{:max_len$}", " ").map_err(io_error)?;
            }
        }
        Ok(())
    }

    fn write_predicate_object(
        &self,
        group: TurtleTripleType,
        subject: &SubjectNodeRef,
        predicate: &IriRef,
        max_len: usize,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        // First, write the predicate
        //
        self.write_predicate(group, predicate, max_len, flags)?;
        // Then, write the object(s) for that predicate (in sorted predictable order)
        //
        self.with_objects(subject, predicate, flags, |c, object, flags| {
            c.write_object(object, max_len, flags)
        })
    }

    fn write_sub_graph(
        &self,
        subject: &SubjectNodeRef,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        self.write_subject(subject.clone(), flags)?;
        self.write_predicates_of_subject(subject.clone(), flags)
    }

    fn write_predicates_of_subject(
        &self,
        subject: SubjectNodeRef,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        self.with_predicates_grouped(&subject, flags, |c, group, predicate, max_len, flags| {
            c.write_predicate_object(group, &subject, predicate, max_len, flags)
        })
    }

    /// Deal with a nested blank node.
    fn write_nested_blank_node(
        &self,
        object: &ObjectNodeRef,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        write!(self, "[").map_err(io_error)?;
        let inner_subject: SubjectNodeRef = self
            .graph
            .statement_factory()
            .object_as_subject(<&Rc<dyn ObjectNode>>::clone(&object).clone())
            .unwrap();
        self.write_sub_graph(&inner_subject, flags)?;
        self.wrote_blank(&inner_subject);
        self.new_line(flags)?;
        write!(self, "]").map_err(io_error)?;
        Ok(())
    }
}

impl<W> Write for &TurtleCursor<'_, W>
where
    W: Write + Sized,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.w.deref().borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.deref().borrow_mut().flush()
    }
}
