/*!
Provides the `TurtleWriter` implementation of the `GraphWriter` trait.

# Example

```rust
use rdftk_io::turtle::writer::{TurtleOptions, TurtleWriter};
use rdftk_io::write_graph_to_string;
use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;
# use rdftk_core::model::graph::GraphRef;
# fn make_graph() -> GraphRef { rdftk_core::simple::graph::graph_factory().graph() }

let mut options = TurtleOptions::default();
options.use_sparql_style = true;
options.nest_blank_nodes = false;
let writer = TurtleWriter::with_base(
    IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/").unwrap()),
    options,
);

let result = write_graph_to_string(&writer, &make_graph());
```

 */

use std::cell::Ref;
use std::io::Write;

use itertools::Itertools;
use rdftk_core::model::graph::{Graph, GraphRef};
use rdftk_core::model::graph::mapping::PrefixMappingRef;
use rdftk_core::model::literal::LiteralRef;
use rdftk_core::model::statement::SubjectNodeRef;
use rdftk_iri::IRIRef;

use crate::common::indenter::Indenter;
use crate::GraphWriter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TurtleOptions {
    pub nest_blank_nodes: bool,
    pub use_sparql_style: bool,
    /// Use the same formatting style as used by the LNKD.tech editor plugin
    /// for the IntelliJ IDEs like Idea and CLion
    pub use_intellij_style: bool,
    /// If provided, any IRI that's written to Turtle that starts with the given
    /// string will be written to Turtle as if it's part of the base namespace.
    pub convert_to_base: Option<String>,
    pub indent_width: usize,
}

#[derive(Debug)]
pub struct TurtleWriter {
    base: Option<String>,
    options: TurtleOptions,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for TurtleOptions {
    fn default() -> Self {
        Self {
            nest_blank_nodes: true,
            use_sparql_style: false,
            use_intellij_style: false,
            convert_to_base: None,
            indent_width: 2,
        }
    }
}

impl TurtleOptions {
    /// Set default options to make the generated Turtle RDF look like it's formatted
    /// by the LNKD.tech plugin that is used in the IntelliJ family of editors such as
    /// Idea and CLion.
    /// This would allow you to load RDF from a git clone and write it back to disk
    /// without causing unnecessary git-diff detectable changes.
    pub fn new_with_intellij_style() -> Self {
        Self {
            use_intellij_style: true,
            indent_width: 4,
            ..Default::default()
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for TurtleWriter {
    fn default() -> Self {
        Self {
            base: None,
            options: Default::default(),
        }
    }
}

impl GraphWriter for TurtleWriter {
    fn write(&self, w: &mut impl Write, graph: &GraphRef) -> rdftk_core::error::Result<()> {
        let graph = graph.borrow();

        //
        // Write out the graph base IRI
        //
        if let Some(base) = &self.base {
            if self.options.use_sparql_style && !self.options.use_intellij_style {
                writeln!(w, "BASE <{}>", base).map_err(io_error)?;
            } else {
                writeln!(w, "@base <{}> .", base).map_err(io_error)?;
            }
        }
        if !self.options.use_intellij_style {
            writeln!(w).map_err(io_error)?;
        }
        //
        // Write all prefix mappings, sort them by prefix
        //
        let mappings = graph.prefix_mappings();
        let mappings = mappings.borrow();
        for (prefix, namespace) in mappings.mappings().sorted() {
            if self.options.use_sparql_style && !self.options.use_intellij_style {
                writeln!(w, "PREFIX {}: <{}>", prefix, namespace).map_err(io_error)?;
            } else {
                writeln!(w, "@prefix {}: <{}> .", prefix, namespace).map_err(io_error)?;
            }
        }
        writeln!(w).map_err(io_error)?;
        //
        // Write statements, start with those where subject is an IRI,
        // sort them by URL so that we keep a consistent result avoiding git-diff to
        // flag certain lines as changed.
        //
        let mut blanks_to_write: Vec<&SubjectNodeRef> = Default::default();
        let mut blanks_written: Vec<SubjectNodeRef> = Default::default();
        for subject in graph.subjects().into_iter().sorted() {
            if subject.is_blank() {
                blanks_to_write.push(subject);
            } else {
                let mut inner_written = self
                    .write_sub_graph(w, subject, &graph, Indenter::with_width(self.options.indent_width))
                    .map_err(io_error)?;
                blanks_written.append(&mut inner_written);
            }
            writeln!(w).map_err(io_error)?;
        }
        //
        // Write statements where subject is a blank node
        //
        blanks_to_write.retain(|subject| !blanks_written.contains(subject));
        for subject in blanks_to_write {
            let _ = self
                .write_sub_graph(w, subject, &graph, Indenter::with_width(self.options.indent_width))
                .map_err(io_error)?;
        }
        Ok(())
    }
}

impl TurtleWriter {
    ///
    /// Create a new writer with the provided options, this is used to override the default
    /// options that are used when calling `Default::default`.
    ///
    pub fn new(options: TurtleOptions) -> Self {
        Self {
            base: None,
            options,
        }
    }
    pub fn with_base(base: IRIRef, options: TurtleOptions) -> Self {
        Self {
            base: Some(base.to_string()),
            options,
        }
    }

    fn write_sub_graph(
        &self,
        w: &mut impl Write,
        subject: &SubjectNodeRef,
        in_graph: &Ref<'_, dyn Graph>,
        indenter: Indenter,
    ) -> std::io::Result<Vec<SubjectNodeRef>> {
        write!(w, "{}", indenter)?;
        let mut indenter = indenter;
        let mut blanks_written: Vec<SubjectNodeRef> = Default::default();
        let mappings = in_graph.prefix_mappings();
        if subject.is_blank() && indenter.depth() == 0 {
            write!(w, "_:{} ", subject.as_blank().unwrap())?;
        } else if subject.is_iri() {
            self.write_iri(w, subject.as_iri().unwrap(), &mappings)?;
        }
        let predicates = in_graph.predicates_for(subject);
        indenter = indenter.indent();
        let mut p_iter = predicates.iter().peekable();
        while let Some(predicate) = p_iter.next() {
            self.write_iri(w, predicate, &mappings)?;
            let objects = in_graph.objects_for(subject, predicate);
            if objects.len() > 1 {
                indenter = indenter.indent();
            }
            let mut o_iter = objects.iter().peekable();
            while let Some(object) = o_iter.next() {
                if object.is_blank() && self.options.nest_blank_nodes {
                    write!(w, "[\n{}", indenter.one())?;
                    let inner_subject: SubjectNodeRef = in_graph
                        .statement_factory()
                        .object_as_subject(
                            <&std::rc::Rc<dyn rdftk_core::model::statement::ObjectNode>>::clone(
                                object,
                            )
                                .clone(),
                        )
                        .unwrap();
                    let mut inner_written =
                        self.write_sub_graph(w, &inner_subject, in_graph, indenter.clone())?;
                    blanks_written.push(inner_subject);
                    blanks_written.append(&mut inner_written);
                    write!(w, "{}]", indenter)?;
                } else if object.is_blank() && !self.options.nest_blank_nodes {
                    write!(w, "_:{}", object.as_blank().unwrap())?;
                } else if object.is_iri() {
                    self.write_iri(w, object.as_iri().unwrap(), &mappings)?;
                } else {
                    self.write_literal(w, object.as_literal().unwrap(), &mappings)?;
                }
                if o_iter.peek().is_some() {
                    writeln!(w, ",")?;
                }
            }
            if p_iter.peek().is_some() {
                write!(w, ";\n{}", indenter)?;
            }
            if objects.len() > 1 {
                indenter = indenter.outdent();
            }
        }
        indenter = indenter.outdent();
        if indenter.depth() == 0 {
            writeln!(w, ".")?;
        } else {
            writeln!(w)?;
        }
        Ok(blanks_written)
    }

    fn write_iri<W: Write>(
        &self,
        w: &mut W,
        iri: &IRIRef,
        mappings: &PrefixMappingRef,
    ) -> std::io::Result<()> {
        if let Some(base) = &self.base {
            let iri = iri.to_string();
            // If we're encountering an IRI whose prefix equals the `convert_to_base` IRI
            // then write it to Turtle as if it's an IRI with the default base.
            if let Some(ref convert_to_base) = self.options.convert_to_base {
                if iri.starts_with(convert_to_base.as_str()) {
                    return write!(w, "<{}> ", &iri[convert_to_base.len()..]);
                }
            }
            if iri.starts_with(base) {
                return write!(w, "<{}> ", &iri[base.len()..]);
            }
        }
        write!(
            w,
            "{} ",
            match mappings.borrow().compress(&iri) {
                None => format!("<{}>", iri),
                Some(qname) => qname.to_string(),
            }
        )
    }

    fn write_literal<W: Write>(
        &self,
        w: &mut W,
        literal: &LiteralRef,
        _mappings: &PrefixMappingRef,
    ) -> std::io::Result<()> {
        // TODO: compress data type IRIs
        write!(w, "{} ", literal)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub fn io_error(e: std::io::Error) -> rdftk_core::error::Error {
    use rdftk_core::error::ErrorKind;
    rdftk_core::error::Error::with_chain(e, ErrorKind::ReadWrite(super::NAME.to_string()))
}
