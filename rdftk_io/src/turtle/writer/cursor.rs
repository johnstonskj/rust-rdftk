#[allow(unused_imports)]
use std::borrow::BorrowMut as DummyDEF;
use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::io::Write;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use itertools::Itertools;

use rdftk_core::model::graph::Graph;
use rdftk_core::model::qname::QName;
use rdftk_core::model::statement::{ObjectNode, ObjectNodeRef, SubjectNodeRef};
use rdftk_iri::IRIRef;

use crate::common::indenter::Indenter;
use crate::turtle::writer::{io_error, utf8_error};
use crate::turtle::writer::options::TurtleOptions;
use crate::turtle::writer::triple_type::TurtleTripleType;

type IsNextObjectBlankNode = bool;
type IsBeingSorted = bool;
type IsLastOfSubject = bool;
type IsLastOfPredicate = bool;

#[derive(Default, Copy, Clone, Debug)]
struct TurtleCursorFlags {
    /// `is_next_object_blank` is true when the next object in the series (also) a blank node?
    /// In that case we do the formatting a bit different, showing ],[` as the separator
    /// between blank nodes.
    is_next_object_blank: IsNextObjectBlankNode,
    /// `is_being_sorted` is true when we're being called by a sorting algorithm which means that
    /// we can only produce content on one (sortable) line, avoid any line-feeds..
    is_being_sorted: IsBeingSorted,
    /// `is_last_of_subject` is true when we're working on the last triple of the current subject,
    /// in which case we have to end a line with a dot instead of a semicolon.
    is_last_of_subject: IsLastOfSubject,
    /// `is_last_of_predicate` is true when the current object is the last object in the collection
    /// of objects for the given `subject + predicate`.
    is_last_of_predicate: IsLastOfPredicate,
}

pub(crate) struct TurtleCursor<'a, W> where W: Write + Sized {
    pub(crate) w: Rc<RefCell<W>>,
    pub(crate) graph: Rc<Ref<'a, dyn Graph>>,
    pub(crate) indenter: RefCell<Indenter>,
    pub(crate) blanks_to_write: RefCell<Vec<SubjectNodeRef>>,
    pub(crate) blanks_written: RefCell<Vec<SubjectNodeRef>>,
    pub(crate) options: TurtleOptions,
}

impl<'a, W: Write + Sized> TurtleCursor<'a, W> {
    pub(crate) fn new(
        w: Rc<RefCell<W>>,
        graph: Rc<Ref<'a, (dyn Graph + 'a)>>,
        options: TurtleOptions,
    ) -> Self {
        let indenter = RefCell::new(Indenter::with_width(options.indent_width));
        let blanks_to_write = RefCell::new(
            graph
                .deref()
                .blank_node_subjects()
                .iter()
                .map(|s| {
                    let x = *s;
                    x.clone()
                })
                .collect()
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
    ///
    /// Get a copy (you clone it first) of the current Cursor but then with the given writer.
    ///
    pub(crate) fn new_with_writer<W2: Write + Sized>(w: Rc<RefCell<W>>, other: &'a TurtleCursor<'a, W2>) -> Self {
        Self {
            w,
            graph: other.graph.clone(),
            indenter: {
                let mut clone = other.indenter.borrow().clone();
                clone.depth = 0;
                RefCell::new(clone)
            },
            blanks_to_write: other.blanks_to_write.clone(),
            blanks_written: other.blanks_written.clone(),
            options: other.options.clone(),
            ..*other
        }
    }

    pub(crate) fn write(&self) -> rdftk_core::error::Result<()> {
        self.write_normal_subjects()?;
        let flags = TurtleCursorFlags::default();
        // The given cursor object collects all the blank-node objects that have not been
        // written to the turtle file yet but have been referred to during the call to
        // `write_normal_subjects` above. Now process those unwritten blank nodes and add
        // them to the end of the file.
        self.write_blank_node_subjects(flags)
    }

    #[allow(unused_results)]
    pub(crate) fn indent(&self) -> &RefCell<Indenter> {
        self.indenter.replace_with(|old| old.indent());
        &self.indenter
    }

    #[allow(unused_results)]
    #[allow(unused)]
    pub(crate) fn outdent(&self) -> &RefCell<Indenter> {
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

    pub(crate) fn write_fmt(&self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        self.w.as_ref().borrow_mut().write_fmt(fmt)
    }

    pub(crate) fn wrote_blank(&self, blank: &SubjectNodeRef) {
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

    pub(crate) fn sorted_subjects(&self) -> Vec<SubjectNodeRef> {
        self.graph
            .node_subjects()
            .into_iter()
            .sorted()
            .cloned()
            .collect::<Vec<SubjectNodeRef>>()
    }

    fn with_node_subjects_do<F>(&self, flags: TurtleCursorFlags, f: F) -> rdftk_core::error::Result<()>
        where F: Fn(&Self, &SubjectNodeRef, TurtleCursorFlags) -> rdftk_core::error::Result<()>
    {
        for subject in self.sorted_subjects() {
            f(self, &subject, flags)?;
        }
        Ok(())
    }

    fn with_unwritten_blank_node_subjects<F>(&self, flags: TurtleCursorFlags, f: F) -> rdftk_core::error::Result<()>
        where F: Fn(&Self, SubjectNodeRef, TurtleCursorFlags) -> rdftk_core::error::Result<()>
    {
        for subject in self.blanks_not_written().into_iter() {
            self.indenter.borrow_mut().depth = 0;
            f(self, subject.clone(), flags)?;
        }
        Ok(())
    }

    fn with_predicates_grouped<F>(&self, subject: &SubjectNodeRef, flags: TurtleCursorFlags, f: F) -> rdftk_core::error::Result<()>
        where F: Fn(&Self, TurtleTripleType, &IRIRef, usize, TurtleCursorFlags) -> rdftk_core::error::Result<()>
    {
        let all_predicates = Vec::from_iter(self.graph.predicates_for(subject));
        let mut count = 0;
        let total_number = all_predicates.len();
        let max_len = 1 + self.max_len_predicates(&all_predicates)?;

        // let max_len = all_predicates.iter().(|)
        //     .fold(std::u16::MIN, |a,b| a.max(b.borrow().));
        for (group, ref preds) in TurtleTripleType::group_predicates(&all_predicates) {
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

    fn max_len_predicates(&self, predicates: &[&IRIRef]) -> rdftk_core::error::Result<usize> {
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

    ///
    /// Iterate through all sorted objects of the given subject and predicate
    ///
    fn with_objects<F>(&self, subject: &SubjectNodeRef, predicate: &IRIRef, flags: TurtleCursorFlags, f: F) -> rdftk_core::error::Result<()>
        where F: Fn(&Self, &ObjectNodeRef, TurtleCursorFlags) -> rdftk_core::error::Result<()>
    {
        let mut objects = self.graph.deref().objects_for(subject, predicate).into_iter().collect_vec();
        let is_collection_of_objects = objects.len() > 1;
        if is_collection_of_objects {
            objects.sort_by_key(|o| {
                self.object_sort_key(o).unwrap_or_default()
            });
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

    fn write_object_content(&self, object: &ObjectNodeRef, flags: TurtleCursorFlags) -> rdftk_core::error::Result<()> {
        if object.is_blank() {
            if self.options.nest_blank_nodes {
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

    ///
    /// Write out a given IRI as Turtle.
    /// Compress any IRI to its "QName" given the supplied set of prefixes and their namespace IRIs.
    /// If we're encountering an IRI whose prefix equals the given (optional) `convert_to_base` IRI
    /// then write it to Turtle as if it's an IRI with the default base.
    ///
    pub(crate) fn write_iri(&self, iri: &IRIRef) -> std::io::Result<()> {
        self.compress_iri(self.w.deref().borrow_mut().deref_mut(), iri)
    }

    pub(crate) fn compress_iri<W2: Write + Sized>(
        &self,
        writer: &mut W2,
        iri: &IRIRef,
    ) -> std::io::Result<()> {
        if let Some(base) = &self.options.base {
            let iri = iri.to_string();
            if let Some(ref convert_to_base) = self.options.convert_to_base {
                if iri.starts_with(convert_to_base.as_str()) {
                    return write!(writer, "<{}> ", &iri[convert_to_base.len()..]);
                }
            }
            if iri.starts_with(base) {
                return write!(writer, "<{}>", &iri[base.len()..]);
            }
        }
        match self.compress(&iri) {
            None => write!(writer, "<{iri}>"),
            Some(_qname) => write!(writer, "{_qname}"),
        }
    }

    ///
    /// Write statements, start with those where subject is an IRI,
    /// sort them by URL so that we keep a consistent result avoiding git-diff to
    /// flag certain lines as changed.
    ///
    fn write_normal_subjects(&self) -> rdftk_core::error::Result<()> {
        let flags = TurtleCursorFlags {
            ..Default::default()
        };
        self.with_node_subjects_do(flags, |c, subject, flags| {
            c.write_sub_graph(subject, flags)?;
            writeln!(c).map_err(io_error)?;
            Ok(())
        })
    }

    ///
    /// Write statements where subject is a blank node
    ///
    fn write_blank_node_subjects(&self, flags: TurtleCursorFlags) -> rdftk_core::error::Result<()> {
        self.with_unwritten_blank_node_subjects(flags, |c, ref subject, flags| {
            c.write_sub_graph(subject, flags)?;
            Ok(())
        })
    }

    fn write_subject(&self, subject: SubjectNodeRef, flags: TurtleCursorFlags) -> rdftk_core::error::Result<()> {
        if subject.is_blank() && self.indenter.borrow().depth() == 0 {
            if flags.is_being_sorted {
                write!(self, " _:{}", subject.as_blank().unwrap()).map_err(io_error)?;
            } else {
                write!(self, "\n_:{}", subject.as_blank().unwrap()).map_err(io_error)?;
            }
        } else if subject.is_iri() {
            self.write_iri(subject.as_iri().unwrap()).map_err(io_error)?;
        }
        let _ = self.indent();
        Ok(())
    }

    ///
    /// Compress an IRI into a qname, if possible.
    ///
    pub(crate) fn compress(&self, iri: &IRIRef) -> Option<QName> {
        self.graph.deref().prefix_mappings().deref().borrow().compress(iri)
    }

    pub(crate) fn write_literal(&self, literal: &ObjectNodeRef) -> std::io::Result<()> {
        // TODO: compress data type IRIs
        if let Some(literal) = literal.as_literal() {
            write!(self, "{}", literal)
        } else {
            write!(self, "ERROR: this is not a literal: {:?}", literal)
        }
    }

    fn write_predicate(
        &self,
        group: TurtleTripleType,
        predicate: &IRIRef,
        max_len: usize,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        //
        // Special treatment for `rdf:type`; show it in turtle as just "a"
        //
        if group == TurtleTripleType::Type {
            return if self.options.place_type_on_subject_line {
                write!(self, " a ").map_err(io_error)
            } else {
                self.new_line(flags)?;
                write!(self, "{:<max_len$}", "a").map_err(io_error)
            };
        }
        //
        // Otherwise, go to the next line and write it as a normal predicate-IRI
        //
        self.new_line(flags)?;

        let buffer = Rc::new(RefCell::from(Vec::<u8>::new()));
        let new_cursor = TurtleCursor::new_with_writer(buffer.clone(), self);
        new_cursor.write_iri(predicate).map_err(io_error)?;
        let pred = String::from_utf8(buffer.take()).map_err(utf8_error)?;
        write!(self, "{:<max_len$}", pred.as_str()).map_err(io_error)
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
            if self.indenter.borrow().depth == 0 {
                write!(self, " .").map_err(io_error)?;
                self.new_line(flags)?;
            } else if !flags.is_last_of_subject {
                write!(self, " ;").map_err(io_error)?;
            }
        } else {
            write!(self, ",").map_err(io_error)?;
            if ! flags.is_next_object_blank {
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
        predicate: &IRIRef,
        max_len: usize,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        //
        // First, write the predicate
        //
        self.write_predicate(group, predicate, max_len, flags)?;
        //
        // Then, write the object(s) for that predicate (in sorted predictable order)
        //
        self.with_objects(subject, predicate, flags, |c, object, flags| {
            c.write_object(object, max_len, flags)
        })
    }

    fn write_sub_graph(&self, subject: &SubjectNodeRef, flags: TurtleCursorFlags) -> rdftk_core::error::Result<()> {
        self.write_subject(subject.clone(), flags)?;
        self.write_predicates_of_subject(subject.clone(), flags)
    }

    fn write_predicates_of_subject(&self, subject: SubjectNodeRef, flags: TurtleCursorFlags) -> rdftk_core::error::Result<()> {
        self.with_predicates_grouped(
            &subject,
            flags,
            |c, group,
             predicate, max_len, flags| {
                c.write_predicate_object(
                    group,
                    &subject,
                    predicate,
                    max_len,
                    flags,
                )
            },
        )
    }

    ///
    /// Deal with a nested blank node.
    ///
    fn write_nested_blank_node(
        &self,
        object: &ObjectNodeRef,
        flags: TurtleCursorFlags,
    ) -> rdftk_core::error::Result<()> {
        write!(self, "[").map_err(io_error)?;
        let inner_subject: SubjectNodeRef = self.graph
            .statement_factory()
            .object_as_subject(
                <&Rc<dyn ObjectNode>>::clone(&object).clone(),
            )
            .unwrap();
        self.write_sub_graph(&inner_subject, flags)?;
        self.wrote_blank(&inner_subject);
        self.new_line(flags)?;
        write!(self, "]").map_err(io_error)?;
        Ok(())
    }
}

impl<'a, W: Write + Sized> Write for &TurtleCursor<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.w.deref().borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.deref().borrow_mut().flush()
    }
}