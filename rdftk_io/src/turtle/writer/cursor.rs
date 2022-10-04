use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::io::Write;
use std::iter::FromIterator;
use std::ops::Deref;

use itertools::Itertools;

use rdftk_core::model::graph::Graph;
use rdftk_core::model::qname::QName;
use rdftk_core::model::statement::{ObjectNodeRef, SubjectNodeRef};
use rdftk_iri::IRIRef;

use crate::common::indenter::Indenter;
use crate::turtle::writer::{io_error, utf8_error};
use crate::turtle::writer::options::TurtleOptions;
use crate::turtle::writer::triple_type::TurtleTripleType;

pub(crate) struct TurtleCursor<'a, W> where W: Write + Sized {
    pub(crate) w: RefCell<W>,
    pub(crate) graph: Ref<'a, dyn Graph>,
    pub(crate) indenter: RefCell<Indenter>,
    pub(crate) blanks_to_write: RefCell<Vec<SubjectNodeRef>>,
    pub(crate) blanks_written: RefCell<Vec<SubjectNodeRef>>,
    pub(crate) options: TurtleOptions,
}

impl<'a, W: Write + Sized> TurtleCursor<'a, W> {
    pub(crate) fn new(
        w: RefCell<W>,
        graph: Ref<'a, (dyn Graph + 'a)>,
        options: TurtleOptions,
    ) -> Self {
        let indenter = RefCell::new(Indenter::with_width(options.indent_width));
        let blanks_to_write = RefCell::new(
            graph
                .deref()
                .borrow()
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
        write!(self.w.borrow_mut(), "\n{}", self.indenter.borrow()).map_err(io_error)
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
        let max_len = 1 + self.max_len_predicates(all_predicates.borrow())?;

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

    pub(crate) fn with_objects<F>(&self, subject: &SubjectNodeRef, predicate: &IRIRef, f: F) -> rdftk_core::error::Result<()>
        where F: Fn(&Self, &ObjectNodeRef, bool) -> rdftk_core::error::Result<()>
    {
        let objects = self.graph.borrow().objects_for(subject, predicate);
        let mut o_iter = objects.iter().peekable();
        while let Some(object) = o_iter.next() {
            f(self, object, o_iter.peek().is_none())?;
        }
        Ok(())
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
    /// Compress an IRI into a qname, if possible.
    ///
    pub(crate) fn compress(&self, iri: &IRIRef) -> Option<QName> {
        self.graph.borrow().prefix_mappings().deref().borrow().compress(iri)
    }
}
