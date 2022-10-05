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
            indenter: other.indenter.clone(),
            blanks_to_write: other.blanks_to_write.clone(),
            blanks_written: other.blanks_written.clone(),
            options: other.options.clone(),
            ..*other
        }
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

    pub(crate) fn new_line(&self) -> rdftk_core::error::Result<()> {
        write!(self, "\n{}", self.indenter.borrow()).map_err(io_error)
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

    pub(crate) fn with_node_subjects_do<F>(&self, f: F) -> rdftk_core::error::Result<()>
        where F: Fn(&Self, &SubjectNodeRef) -> rdftk_core::error::Result<()>
    {
        for subject in self.sorted_subjects() {
            f(self, &subject)?;
        }
        Ok(())
    }

    pub(crate) fn with_unwritten_blank_node_subjects<F>(&mut self, f: F) -> rdftk_core::error::Result<()>
        where F: Fn(&mut Self, SubjectNodeRef) -> rdftk_core::error::Result<()>
    {
        for subject in self.blanks_not_written().into_iter() {
            f(self, subject.clone())?;
        }
        Ok(())
    }

    pub(crate) fn with_predicates_grouped<F>(&self, subject: &SubjectNodeRef, f: F) -> rdftk_core::error::Result<()>
        where F: Fn(&Self, TurtleTripleType, &IRIRef, usize, bool) -> rdftk_core::error::Result<()>
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
                f(self, group, predicate, max_len, count == total_number)?;
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
    pub(crate) fn with_objects<F>(&self, subject: &SubjectNodeRef, predicate: &IRIRef, f: F) -> rdftk_core::error::Result<()>
        where F: Fn(&Self, &ObjectNodeRef, bool) -> rdftk_core::error::Result<()>
    {
        let mut objects = self.graph.deref().objects_for(subject, predicate).into_iter().collect_vec();
        objects.sort_by_key(|o| {
            self.object_sort_key(o).unwrap_or_default()
        });
        let mut o_iter = objects.iter().peekable();
        while let Some(object) = o_iter.next() {
            f(self, object, o_iter.peek().is_none())?;
        }
        Ok(())
    }

    fn object_sort_key(&self, object: &ObjectNodeRef) -> rdftk_core::error::Result<String> {
        let buffer = Rc::new(RefCell::from(Vec::<u8>::new()));
        let new_cursor = TurtleCursor::new_with_writer(buffer.clone(), self);
        new_cursor.write_object_content(object)?;
        String::from_utf8(buffer.take()).map_err(utf8_error)
    }

    pub(crate) fn write_object_content(&self, object: &ObjectNodeRef) -> rdftk_core::error::Result<()> {
        if object.is_blank() {
            if self.options.nest_blank_nodes {
                self.write_nested_blank_node(object)?;
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

    pub(crate) fn write_subject(&self, subject: SubjectNodeRef) -> rdftk_core::error::Result<()> {
        if subject.is_blank() && self.indenter.borrow().depth() == 0 {
            write!(self, "\n_:{}", subject.as_blank().unwrap()).map_err(io_error)?;
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

    pub(crate) fn write_predicate(
        &self,
        group: TurtleTripleType,
        predicate: &IRIRef,
        max_len: usize,
    ) -> rdftk_core::error::Result<()> {
        //
        // Special treatment for `rdf:type`; show it in turtle as just "a"
        //
        if group == TurtleTripleType::Type {
            return if self.options.place_type_on_subject_line {
                write!(self, " a ").map_err(io_error)
            } else {
                self.new_line()?;
                write!(self, "{:<max_len$}", "a").map_err(io_error)
            };
        }
        //
        // Otherwise, go to the next line and write it as a normal predicate-IRI
        //
        self.new_line()?;

        let buffer = Rc::new(RefCell::from(Vec::<u8>::new()));
        let new_cursor = TurtleCursor::new_with_writer(buffer.clone(), self);
        new_cursor.write_iri(predicate).map_err(io_error)?;
        let pred = String::from_utf8(buffer.take()).map_err(utf8_error)?;
        write!(self, "{:<max_len$}", pred.as_str()).map_err(io_error)
    }

    pub(crate) fn write_object(
        &self,
        object: &ObjectNodeRef,
        max_len: usize,
        is_last_of_predicate: bool,
        is_last_of_subject: bool,
    ) -> rdftk_core::error::Result<()> {
        self.write_object_content(object)?;
        if is_last_of_predicate {
            if is_last_of_subject {
                let _ = self.outdent();
            }
            if self.indenter.borrow().depth == 0 {
                write!(self, " .").map_err(io_error)?;
            } else {
                write!(self, " ;").map_err(io_error)?;
            }
        } else {
            write!(self, ",").map_err(io_error)?;
            self.new_line()?;
            write!(self, "{:max_len$}", " ").map_err(io_error)?;
        }
        Ok(())
    }

    pub(crate) fn write_predicate_object(
        &self,
        group: TurtleTripleType,
        subject: &SubjectNodeRef,
        predicate: &IRIRef,
        max_len: usize,
        is_last_of_subject: bool,
    ) -> rdftk_core::error::Result<()> {
        //
        // First, write the predicate
        //
        self.write_predicate(group, predicate, max_len)?;
        //
        // Then, write the object(s) for that predicate (in sorted predictable order)
        //
        self.with_objects(subject, predicate, |c, object, is_last| {
            c.write_object(object, max_len, is_last, is_last_of_subject)
        })
    }

    pub(crate) fn write_sub_graph(&self, subject: &SubjectNodeRef) -> rdftk_core::error::Result<()> {
        self.write_subject(subject.clone())?;
        self.write_predicates_of_subject(subject.clone())
    }

    fn write_predicates_of_subject(&self, subject: SubjectNodeRef) -> rdftk_core::error::Result<()> {
        self.with_predicates_grouped(
            &subject,
            |c, group,
             predicate, max_len, is_last| {
                c.write_predicate_object(
                    group,
                    &subject,
                    predicate,
                    max_len,
                    is_last,
                )
            },
        )
    }

    ///
    /// Deal with a nested blank node.
    ///
    fn write_nested_blank_node(&self, object: &ObjectNodeRef) -> rdftk_core::error::Result<()> {
        write!(self, " [").map_err(io_error)?;
        let inner_subject: SubjectNodeRef = self.graph
            .statement_factory()
            .object_as_subject(
                <&Rc<dyn ObjectNode>>::clone(&object).clone(),
            )
            .unwrap();
        self.write_sub_graph(&inner_subject)?;
        self.wrote_blank(&inner_subject);
        self.new_line()?;
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