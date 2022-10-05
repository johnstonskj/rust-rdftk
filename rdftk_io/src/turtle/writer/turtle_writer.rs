#[allow(unused_imports)]
use std::borrow::BorrowMut as DummyABC;
use std::cell::RefCell;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

use itertools::Itertools;

use rdftk_core::model::graph::GraphRef;
use rdftk_iri::IRIRef;

use crate::GraphWriter;
use crate::turtle::writer;
use crate::turtle::writer::cursor::TurtleCursor;
use crate::turtle::writer::io_error;
use crate::turtle::writer::options::TurtleOptions;

#[derive(Debug, Default)]
pub struct TurtleWriter {
    pub options: TurtleOptions,
}

impl GraphWriter for TurtleWriter {
    fn write(&self, w: &mut impl Write, graph: &GraphRef) -> rdftk_core::error::Result<()> {
        self.write_base_iri(w)?;
        self.write_prefixes(w, graph)?;

        let mut cursor = TurtleCursor::new(
            Rc::new(RefCell::from(w)),
            Rc::from(graph.deref().borrow()),
            self.options.clone(),
        );

        self.write_normal_subjects(&mut cursor)?;
        self.write_blank_node_subjects(&mut cursor)
    }
}

impl TurtleWriter {
    ///
    /// Create a new writer with the provided options, this is used to override the default
    /// options that are used when calling `Default::default`.
    ///
    pub fn new(options: TurtleOptions) -> Self {
        Self {
            options,
        }
    }
    pub fn with_base(base: IRIRef, options: TurtleOptions) -> Self {
        Self::new(options.with_base(base))
    }

    pub fn with_indent_with(width: u16) -> Self {
        Self::new(TurtleOptions::default().with_indent_width(width))
    }

    ///
    /// Write out the graph base IRI in either turtle
    /// style (as '@base ..') or SPARQL style (as 'BASE ...')
    ///
    fn write_base_iri(&self, w: &mut impl Write) -> rdftk_core::error::Result<()> {
        let TurtleOptions { use_sparql_style, use_intellij_style, .. } = self.options;
        if let Some(base) = &self.options.base {
            if use_sparql_style && !use_intellij_style {
                writeln!(w, "BASE <{}>", base).map_err(io_error)?;
            } else {
                writeln!(w, "@base <{}> .", base).map_err(io_error)?;
            }
        }
        if !use_intellij_style {
            writeln!(w).map_err(io_error)?;
        }
        Ok(())
    }

    ///
    /// Write all prefix mappings, sort them by prefix to avoid
    /// random order and unnecessary changes in git
    ///
    fn write_prefixes(&self, w: &mut impl Write, graph: &GraphRef) -> rdftk_core::error::Result<()> {
        let mappings = graph.deref().borrow().prefix_mappings();
        let mappings = mappings.try_borrow().map_err(writer::borrow_error)?;
        for (prefix, namespace) in mappings.mappings().sorted() {
            if self.options.use_sparql_style && !self.options.use_intellij_style {
                writeln!(w, "PREFIX {}: <{}>", prefix, namespace).map_err(io_error)?;
            } else {
                writeln!(w, "@prefix {}: <{}> .", prefix, namespace).map_err(io_error)?;
            }
        }
        writeln!(w).map_err(io_error)
    }

    ///
    /// Write statements, start with those where subject is an IRI,
    /// sort them by URL so that we keep a consistent result avoiding git-diff to
    /// flag certain lines as changed.
    ///
    fn write_normal_subjects(&self, cursor: &mut TurtleCursor<'_, impl Write + Sized>) -> rdftk_core::error::Result<()> {
        cursor.with_node_subjects_do(|c, subject| {
            c.write_sub_graph(subject)?;
            writeln!(c).map_err(io_error)?;
            Ok(())
        })
    }

    ///
    /// Write statements where subject is a blank node
    ///
    fn write_blank_node_subjects(
        &self,
        cursor: &mut TurtleCursor<'_, impl Write + Sized>,
    ) -> rdftk_core::error::Result<()> {
        cursor.with_unwritten_blank_node_subjects(|c, ref subject| {
            c.write_sub_graph(subject)?;
            Ok(())
        })
    }

}
