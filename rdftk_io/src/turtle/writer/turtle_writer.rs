use std::cell::RefCell;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

use itertools::Itertools;

use rdftk_core::model::graph::GraphRef;
use rdftk_core::model::statement::{ObjectNode, ObjectNodeRef, SubjectNodeRef};
use rdftk_iri::IRIRef;

use crate::GraphWriter;
use crate::turtle::writer;
use crate::turtle::writer::cursor::TurtleCursor;
use crate::turtle::writer::{io_error, utf8_error};
use crate::turtle::writer::options::TurtleOptions;
use crate::turtle::writer::triple_type::TurtleTripleType;

#[derive(Debug)]
pub struct TurtleWriter {
    pub options: TurtleOptions,
}

impl Default for TurtleWriter {
    fn default() -> Self {
        Self {
            options: Default::default(),
        }
    }
}

impl GraphWriter for TurtleWriter {
    fn write(&self, w: &mut impl Write, graph: &GraphRef) -> rdftk_core::error::Result<()> {
        self.write_base_iri(w)?;
        self.write_prefixes(w, graph)?;


        let mut cursor = TurtleCursor::new(
            RefCell::from(w),
            graph.deref().borrow(),
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
    /// Write out the graph base IRI
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
            self.write_sub_graph(c, subject)?;
            writeln!(c.w.borrow_mut()).map_err(io_error)?;
            Ok(())
        })
    }

    ///
    /// Write statements where subject is a blank node
    ///
    fn write_blank_node_subjects(&self, cursor: &mut TurtleCursor<'_, impl Write + Sized>) -> rdftk_core::error::Result<()> {
        cursor.with_unwritten_blank_node_subjects(|c, ref subject| {
            self.write_sub_graph(c, subject)?;
            Ok(())
        })
    }

    fn write_sub_graph<'a>(
        &'a self,
        cursor: &'a TurtleCursor<'a, impl Write + Sized>,
        subject: &SubjectNodeRef,
    ) -> rdftk_core::error::Result<()> {
        self.write_subject(cursor, subject.clone())?;
        self.write_predicates_of_subject(cursor, subject.clone())
    }

    fn write_subject(
        &self,
        cursor: &TurtleCursor<'_, impl Write + Sized>,
        subject: SubjectNodeRef,
    ) -> rdftk_core::error::Result<()> {
        if subject.is_blank() && cursor.indenter.borrow().depth() == 0 {
            write!(cursor.w.borrow_mut(), "\n_:{}", subject.as_blank().unwrap()).map_err(io_error)?;
        } else if subject.is_iri() {
            self.write_iri(cursor, subject.as_iri().unwrap()).map_err(io_error)?;
        }
        let _ = cursor.indent();
        Ok(())
    }

    fn write_predicates_of_subject(
        &self,
        cursor: &TurtleCursor<'_, impl Write + Sized>,
        subject: SubjectNodeRef,
    ) -> rdftk_core::error::Result<()> {
        cursor.with_predicates_grouped(
            &subject,
            |c, group,
             predicate, max_len, is_last| {
                self.write_predicate_object(
                    c,
                    group,
                    &subject,
                    predicate,
                    max_len,
                    is_last,
                )
            },
        )
    }

    fn write_predicate_object<W: Write + Sized>(
        &self,
        cursor: &TurtleCursor<'_, W>,
        group: TurtleTripleType,
        subject: &SubjectNodeRef,
        predicate: &IRIRef,
        max_len: usize,
        is_last_of_subject: bool,
    ) -> rdftk_core::error::Result<()> {
        //
        // First, write the predicate
        //
        self.write_predicate(cursor, group, predicate, max_len)?;
        //
        // Then, write the object(s) for that predicate
        //
        cursor.with_objects(subject, predicate, |c, object, is_last| {
            self.write_object(c, object, is_last, is_last_of_subject)
        })
    }

    fn write_predicate<W: Write + Sized>(
        &self,
        cursor: &TurtleCursor<'_, W>,
        group: TurtleTripleType,
        predicate: &IRIRef,
        max_len: usize,
    ) -> rdftk_core::error::Result<()> {
        //
        // Special treatment for `rdf:type`; show it in turtle as just "a"
        //
        if group == TurtleTripleType::Type {
            return if self.options.place_type_on_subject_line {
                write!(cursor.w.borrow_mut(), " a ").map_err(io_error)
            } else {
                cursor.new_line()?;
                write!(cursor.w.borrow_mut(), "{:<max_len$}", "a").map_err(io_error)
            }
        }
        //
        // Otherwise, go to the next line and write it as a normal predicate-IRI
        //
        cursor.new_line()?;

        let mut buffer = Vec::<u8>::new();
        cursor.compress_iri(&mut buffer, predicate).map_err(io_error)?;
        let pred = String::from_utf8(buffer).map_err(utf8_error)?;
        write!(cursor.w.borrow_mut(), "{:<max_len$}", pred.as_str()).map_err(io_error)
    }

    fn write_object<W: Write + Sized>(
        &self,
        cursor: &TurtleCursor<'_, W>,
        object: &ObjectNodeRef,
        is_last_of_predicate: bool,
        is_last_of_subject: bool,
    ) -> rdftk_core::error::Result<()> {
        if object.is_blank() {
            if self.options.nest_blank_nodes {
                self.write_nested_blank_node(cursor, object)?;
            } else {
                write!(cursor.w.borrow_mut(), "_:{}", object.as_blank().unwrap()).map_err(io_error)?;
            }
        } else if object.is_iri() {
            self.write_iri(cursor, object.as_iri().unwrap()).map_err(io_error)?;
        } else {
            self.write_literal(cursor, object).map_err(io_error)?;
        }
        if !is_last_of_predicate {
            write!(cursor.w.borrow_mut(), ", ").map_err(io_error)?;
        } else {
            if is_last_of_subject {
                let _ = cursor.outdent();
            }
            if cursor.indenter.borrow().depth == 0 {
                write!(cursor.w.borrow_mut(), " .").map_err(io_error)?;
            } else {
                write!(cursor.w.borrow_mut(), " ;").map_err(io_error)?;
            }
        }
        Ok(())
    }

    ///
    /// Deal with a nested blank node.
    ///
    fn write_nested_blank_node<W: Write + Sized>(
        &self,
        cursor: &TurtleCursor<'_, W>,
        object: &ObjectNodeRef,
    ) -> rdftk_core::error::Result<()> {
        write!(cursor.w.borrow_mut(), " [").map_err(io_error)?;
        let inner_subject: SubjectNodeRef = cursor.graph
            .statement_factory()
            .object_as_subject(
                <&Rc<dyn ObjectNode>>::clone(&object).clone(),
            )
            .unwrap();
        self.write_sub_graph(cursor, &inner_subject)?;
        cursor.wrote_blank(&inner_subject);
        cursor.new_line()?;
        write!(cursor.w.borrow_mut(), "]").map_err(io_error)?;
        Ok(())
    }

    ///
    /// Write out a given IRI as Turtle.
    /// Compress any IRI to its "QName" given the supplied set of prefixes and their namespace IRIs.
    /// If we're encountering an IRI whose prefix equals the given (optional) `convert_to_base` IRI
    /// then write it to Turtle as if it's an IRI with the default base.
    ///
    fn write_iri<W: Write + Sized>(
        &self,
        cursor: &TurtleCursor<'_, W>,
        iri: &IRIRef,
    ) -> std::io::Result<()> {
        cursor.compress_iri(cursor.w.borrow_mut().by_ref(), iri)
    }

    fn write_literal<W: Write + Sized>(
        &self,
        cursor: &TurtleCursor<'_, W>,
        literal: &ObjectNodeRef,
    ) -> std::io::Result<()> {
        // TODO: compress data type IRIs
        if let Some(literal) = literal.as_literal() {
            write!(cursor.w.borrow_mut(), "{}", literal)
        } else {
            write!(cursor.w.borrow_mut(), "ERROR: this is not a literal: {:?}", literal)
        }
    }
}
