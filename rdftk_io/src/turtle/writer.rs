use crate::common::indenter::Indenter;
use crate::GraphWriter;
use itertools::Itertools;
use objio::{impl_has_options, HasOptions, ObjectWriter};
use rdftk_core::error::{Error, Result};
use rdftk_core::model::graph::Graph;
use rdftk_core::model::statement::{ObjectNode, SubjectNode};
use rdftk_iri::Iri;
use std::cell::RefCell;
use std::io::Write;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Options to change the behavior of the [`TurtleWriter`] implementation.
///
#[derive(Clone, Debug)]
pub struct TurtleWriterOptions {
    id_base: Option<Iri>,
    nest_blank_nodes: bool,
    use_sparql_style: bool,
    use_intellij_style: bool,
    place_type_on_subject_line: bool,
    convert_to_id_base: Option<Iri>,
    convert_base: Vec<(Iri, Iri)>,
    indent_width: usize,
}

///
/// An implementation of `ObjectWriter` for Graphs.
///
#[derive(Clone, Debug, Default)]
pub struct TurtleWriter {
    options: TurtleWriterOptions,
    context: RefCell<WriterContext>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default)]
struct WriterStatusFlags {
    ///
    /// `is_next_object_blank` is true when the next object in the series (also)
    /// a blank node? In that case we do the formatting a bit different,
    /// showing ],[` as the separator between blank nodes.
    ///
    is_next_object_blank: bool,

    ///
    /// `is_being_sorted` is true when we're being called by a sorting algorithm
    /// which means that we can only produce content on one (sortable) line,
    /// avoid any line-feeds..
    ///
    is_being_sorted: bool,

    ///
    /// `is_last_of_subject` is true when we're working on the last triple of
    /// the current subject, in which case we have to end a line with a dot
    /// instead of a semicolon.
    ///
    is_last_of_subject: bool,

    ///
    /// `is_last_of_predicate` is true when the current object is the last
    /// object in the collection of objects for the given `subject +
    /// predicate`.
    ///
    is_last_of_predicate: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum PredicateGroupOrdering {
    ///
    /// `rdf:type` triples come first and are written as `a`.
    ///
    Type,
    ///
    /// `rdfs:label` and/or `skos:prefLabel` triples come second.
    ///
    Label,
    ///
    /// `rdfs:comment` and/or `dc:description` triples come third.
    ///
    Comment,
    ///
    /// Everything else comes last.
    ///
    Other,
}

#[derive(Clone, Debug, Default)]
struct WriterContext {
    indenter: Indenter,
    // TODO: make these BlankNode vectors
    // TODO: make these references?
    blanks_to_write: Vec<SubjectNode>,
    blanks_written: Vec<SubjectNode>,
}

// ------------------------------------------------------------------------------------------------
// Implementations > Options
// ------------------------------------------------------------------------------------------------

impl Default for TurtleWriterOptions {
    fn default() -> Self {
        Self {
            id_base: None,
            nest_blank_nodes: true,
            use_sparql_style: false,
            use_intellij_style: false,
            place_type_on_subject_line: false,
            convert_to_id_base: None,
            convert_base: Vec::new(),
            indent_width: 2,
        }
    }
}

impl TurtleWriterOptions {
    ///
    /// Return a new instance of the given `TurtleOptions` where the `id_base` is set to the given
    /// Iri which will instruct the `TurtleWriter` to generate a `@base <id_base>` or `BASE <id_base>`
    /// statement at the top of the file.
    ///
    pub fn with_id_base(self, id_base: Iri) -> Self {
        Self {
            id_base: Some(id_base.clone()),
            ..self
        }
    }

    ///
    /// Set default options to make the generated Turtle RDF look like it's formatted
    /// by the LNKD.tech plugin that is used in the IntelliJ family of editors such as
    /// Idea and CLion.
    ///
    /// This would allow you to load RDF from a git clone and write it back to disk
    /// without causing unnecessary git-diff detectable changes.
    ///
    pub fn with_intellij_style(self) -> Self {
        Self {
            use_intellij_style: true,
            indent_width: 4,
            ..self
        }
    }

    pub fn with_sparql_style(self) -> Self {
        Self {
            use_sparql_style: true,
            ..self
        }
    }

    pub fn with_indent_width(self, indent_width: usize) -> Self {
        Self {
            indent_width,
            ..self
        }
    }

    pub fn with_nested_blank_nodes(self) -> Self {
        Self {
            nest_blank_nodes: true,
            ..self
        }
    }

    pub fn without_nested_blank_nodes(self) -> Self {
        Self {
            nest_blank_nodes: false,
            ..self
        }
    }

    pub fn id_base(&self) -> Option<&Iri> {
        self.id_base.as_ref()
    }

    pub fn set_id_base(&mut self, id_base: Iri) {
        self.id_base = Some(id_base);
    }

    pub fn unset_id_base(&mut self) {
        self.id_base = None;
    }

    pub fn nest_blank_nodes(&self) -> bool {
        self.nest_blank_nodes
    }

    pub fn set_nest_blank_nodes(&mut self, nest_blank_nodes: bool) {
        self.nest_blank_nodes = nest_blank_nodes;
    }

    pub fn use_sparql_style(&self) -> bool {
        self.use_sparql_style
    }

    pub fn set_use_sparql_style(&mut self, use_sparql_style: bool) {
        self.use_sparql_style = use_sparql_style;
    }

    ///
    /// Use the same formatting style as used by the LNKD.tech editor plugin
    /// for the IntelliJ IDEs like Idea and CLion
    ///
    pub fn use_intellij_style(&self) -> bool {
        self.use_intellij_style
    }

    pub fn set_use_intellij_style(&mut self, use_intellij_style: bool) {
        self.use_intellij_style = use_intellij_style;
    }

    ///
    /// Some prefer to show the `rdf:type type` (or `a type`) statement on the same line as
    /// the subject Iri.
    ///
    pub fn place_type_on_subject_line(&self) -> bool {
        self.place_type_on_subject_line
    }

    pub fn set_place_type_on_subject_line(&mut self, place_type_on_subject_line: bool) {
        self.place_type_on_subject_line = place_type_on_subject_line;
    }

    ///
    /// If provided, any Iri that's written to Turtle that starts with the given
    /// Iri will be written to Turtle as if it's part of the base namespace.
    ///
    pub fn convert_to_id_base(&self) -> Option<&Iri> {
        self.convert_to_id_base.as_ref()
    }

    pub fn set_convert_to_id_base(&mut self, convert_to_id_base: Iri) {
        self.convert_to_id_base = Some(convert_to_id_base);
    }

    pub fn unset_convert_to_id_base(&mut self) {
        self.convert_to_id_base = None;
    }

    ///
    /// If provided, any Iri that's written to Turtle that starts with the given
    /// Iri will be converted with the provided second base Iri.
    ///
    pub fn convert_base(&self) -> &Vec<(Iri, Iri)> {
        &self.convert_base
    }

    ///
    /// Retrieve the indentation width, or the number of spaces to insert at each level of
    /// indentation.
    ///
    pub fn indent_width(&self) -> usize {
        self.indent_width
    }

    ///
    /// Set the indentation width, or the number of spaces to insert at each level of
    /// indentation. This will panic if `indent_width` is zero.
    ///
    pub fn set_indent_width(&mut self, indent_width: usize) {
        assert!(indent_width > 0);
        self.indent_width = indent_width;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > Writer
// ------------------------------------------------------------------------------------------------

impl_has_options!(TurtleWriter, TurtleWriterOptions);

impl ObjectWriter<Graph> for TurtleWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &Graph) -> Result<()>
    where
        W: Write,
    {
        {
            let mut context_mut = self.context.borrow_mut();
            context_mut.indenter =
                Indenter::default().with_default_indent_width(self.options.indent_width());
            context_mut.blanks_to_write = graph
                .blank_node_subjects()
                .iter()
                .map(|s| {
                    let x = *s;
                    x.clone()
                })
                .collect();
            context_mut.blanks_written = Default::default();
        }

        self.write_base_iri(w)?;
        self.write_prefixes(w, graph)?;
        let flags = WriterStatusFlags::default();
        self.write_normal_subjects(w, graph, flags)?;
        // The given cursor object collects all the blank-node objects that have not
        // been written to the turtle file yet but have been referred to during
        // the call to `write_normal_subjects` above. Now process those
        // unwritten blank nodes and add them to the end of the file.
        self.write_blank_node_subjects(w, graph, flags)
    }
}

impl GraphWriter for TurtleWriter {}

impl TurtleWriter {
    fn indent(&self) {
        let context = self.context.borrow();
        context.indenter.indent();
    }

    fn outdent(&self) {
        let context = self.context.borrow();
        context.indenter.outdent();
    }

    fn new_line<W: Write>(&self, w: &mut W, flags: WriterStatusFlags) -> Result<()> {
        if flags.is_being_sorted {
            Ok(write!(w, " ")?)
        } else {
            Ok(write!(w, "\n{}", self.context.borrow().indenter)?)
        }
    }

    fn wrote_blank(&self, blank: &SubjectNode) {
        assert!(blank.is_blank());
        self.context.borrow_mut().blanks_written.push(blank.clone());
    }

    //fn blanks_not_written(&self) -> HashSet<SubjectNode> {
    //    let context = self.context.borrow();
    //    let blanks_written = &context.blanks_written;
    //    context
    //        .blanks_to_write
    //        .iter()
    //        .filter(|subject| !blanks_written.contains(subject))
    //        .cloned()
    //        .collect()
    //}

    fn sorted_subjects(&self, graph: &Graph) -> Vec<SubjectNode> {
        graph
            .node_subjects()
            .into_iter()
            .sorted()
            .cloned()
            .collect::<Vec<SubjectNode>>()
    }

    /// Write out the graph base Iri in either turtle
    /// style (as '@base ..') or SPARQL style (as 'BASE ...')
    fn write_base_iri<W: Write>(&self, w: &mut W) -> Result<()> {
        if let Some(base) = &self.options.id_base() {
            if self.options.use_sparql_style() && !self.options.use_intellij_style() {
                writeln!(w, "BASE <{}>", base.to_string().as_str())?;
            } else {
                writeln!(w, "@base <{}> .", base.to_string().as_str())?;
            }
        }
        if !self.options.use_intellij_style() {
            writeln!(w)?;
        }
        Ok(())
    }

    /// Write all prefix mappings, sort them by prefix to avoid
    /// random order and unnecessary changes in git
    fn write_prefixes<W: Write>(&self, w: &mut W, graph: &Graph) -> Result<()> {
        let mappings = graph.prefix_mappings();
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
                writeln!(w, "PREFIX {prefix}: <{namespace_str}>")?;
            } else {
                writeln!(w, "@prefix {prefix}: <{namespace_str}> .")?;
            }
        }
        Ok(writeln!(w)?)
    }

    //fn with_unwritten_blank_node_subjects<W, F>(
    //    &self,
    //    w: &mut W,
    //    graph: &Graph,
    //    flags: WriterStatusFlags,
    //    f: F,
    //) -> rdftk_core::error::Result<()>
    //where
    //    F: Fn(
    //        &Self,
    //        &mut W,
    //        &Graph,
    //        SubjectNode,
    //        WriterStatusFlags,
    //    ) -> rdftk_core::error::Result<()>,
    //{
    //    for subject in self.blanks_not_written().into_iter() {
    //        self.context.borrow_mut().indenter.reset_depth();
    //        f(self, w, graph, subject.clone(), flags)?;
    //    }
    //    Ok(())
    //}

    fn max_len_predicates(&self, graph: &Graph, predicates: &[&Iri]) -> Result<usize> {
        let all_predicates_as_strings = predicates
            .iter()
            .map(|iri| self.compress_iri(graph, iri))
            .collect::<Result<Vec<String>>>()?
            .iter()
            .fold(0, |a, b| a.max(b.len()));
        Ok(all_predicates_as_strings)
    }

    fn object_sort_key(&self, graph: &Graph, object: &ObjectNode) -> Result<String> {
        let mut buffer = Vec::<u8>::new();
        let new_writer = Self::default().with_options(self.options.clone());
        let flags = WriterStatusFlags {
            is_being_sorted: true,
            ..Default::default()
        };
        new_writer.write_object_content(&mut buffer, graph, object, flags)?;
        Ok(String::from_utf8(buffer)?)
    }

    fn write_object_content<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        object: &ObjectNode,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        if object.is_blank() {
            if self.options.nest_blank_nodes() {
                self.write_nested_blank_node(w, graph, object, flags)?;
            } else {
                write!(w, "_:{}", object.as_blank().unwrap())?;
            }
        } else if object.is_resource() {
            self.write_iri(w, graph, object.as_resource().unwrap())?;
        } else {
            self.write_literal(w, object)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn write_iri<W: Write>(&self, w: &mut W, graph: &Graph, iri: &Iri) -> Result<()> {
        Ok(write!(w, "{}", self.compress_iri(graph, iri)?)?)
    }

    /// Compress any Iri to its "QName" given the supplied set of prefixes and
    /// their namespace Iris. If we're encountering an Iri whose prefix
    /// equals the given (optional) `convert_to_base` Iri then write it to
    /// Turtle as if it's an Iri with the default base.
    fn compress_iri(&self, graph: &Graph, iri: &Iri) -> Result<String> {
        let mut iri_str = iri.to_string();
        if let Some(id_base) = &self.options.id_base() {
            if let Some(ref convert_to_id_base) = self.options.convert_to_id_base() {
                let target_id_base = convert_to_id_base.to_string();
                if iri_str.starts_with(target_id_base.as_str()) {
                    return Ok(format!("<{}>", &iri_str[target_id_base.len()..]));
                }
            }
            let id_base_str = id_base.to_string();
            if iri_str.starts_with(id_base_str.as_str()) {
                return Ok(format!("<{}>", &iri_str[id_base_str.len()..]));
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
        let iri = Iri::from_str(iri_str.as_str())?;
        Ok(match graph.prefix_mappings().compress(&iri) {
            None => format!("<{iri}>"),
            Some(_qname) => format!("{_qname}"),
        })
    }

    /// Write statements, start with those where subject is an Iri,
    /// sort them by URL so that we keep a consistent result avoiding git-diff
    /// to flag certain lines as changed.
    fn write_normal_subjects<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        for subject in self.sorted_subjects(graph) {
            self.write_sub_graph(w, graph, &subject, flags)?;
            writeln!(w)?;
        }
        Ok(())
    }

    /// Write statements where subject is a blank node
    fn write_blank_node_subjects<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        let context = self.context.borrow();
        for subject in context.blanks_to_write.iter() {
            context.indenter.reset_depth();
            self.write_sub_graph(w, graph, subject, flags)?;
        }
        Ok(())
    }

    fn write_subject<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        subject: &SubjectNode,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        let depth = self.context.borrow().indenter.depth();
        if subject.is_blank() && depth == 0 {
            if flags.is_being_sorted {
                write!(w, " _:{}", subject.as_blank().unwrap())?;
            } else {
                write!(w, "\n_:{}", subject.as_blank().unwrap())?;
            }
        } else if subject.is_resource() {
            self.write_iri(w, graph, subject.as_resource().unwrap())?;
        }
        self.indent();
        Ok(())
    }

    fn write_literal<W: Write>(&self, w: &mut W, literal: &ObjectNode) -> Result<()> {
        // TODO: compress data type Iris
        Ok(if let Some(literal) = literal.as_literal() {
            write!(w, "{}", literal)
        } else {
            write!(w, "ERROR: this is not a literal: {:?}", literal)
        }?)
    }

    fn write_predicate<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        group: PredicateGroupOrdering,
        predicate: &Iri,
        max_len: usize,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        // Special treatment for `rdf:type`; show it in turtle as just "a"
        //
        if group == PredicateGroupOrdering::Type {
            return if self.options.place_type_on_subject_line() {
                Ok(write!(w, " a ")?)
            } else {
                self.new_line(w, flags)?;
                Ok(write!(w, "{:<max_len$}", "a")?)
            };
        }
        // Otherwise, go to the next line and write it as a normal predicate-Iri
        //
        self.new_line(w, flags)?;
        let pred = self.compress_iri(graph, predicate)?;
        Ok(write!(w, "{:<max_len$}", pred.as_str())?)
    }

    fn write_object<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        object: &ObjectNode,
        max_len: usize,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        self.write_object_content(w, graph, object, flags)?;
        if flags.is_last_of_predicate {
            if flags.is_last_of_subject {
                self.outdent();
            }
            let depth = self.context.borrow().indenter.depth();
            if depth == 0 {
                write!(w, " .")?;
                self.new_line(w, flags)?;
            } else if !flags.is_last_of_subject {
                write!(w, " ;")?;
            }
        } else {
            write!(w, ",")?;
            if !flags.is_next_object_blank {
                self.new_line(w, flags)?;
                write!(w, "{:max_len$}", " ")?;
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn write_predicate_object<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        group: PredicateGroupOrdering,
        subject: &SubjectNode,
        predicate: &Iri,
        max_len: usize,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        // First, write the predicate
        self.write_predicate(w, graph, group, predicate, max_len, flags)?;

        // Then, write the object(s) for that predicate (in sorted predictable order)
        let mut objects = graph
            .objects_for(subject, predicate)
            .into_iter()
            .collect_vec();
        let is_collection_of_objects = objects.len() > 1;
        if is_collection_of_objects {
            objects.sort_by_key(|o| self.object_sort_key(graph, o).unwrap_or_default());
        }
        let mut o_iter = objects.iter().peekable();
        while let Some(object) = o_iter.next() {
            let next_object = o_iter.peek();
            let flags = WriterStatusFlags {
                is_next_object_blank: next_object.is_some() && next_object.unwrap().is_blank(),
                is_last_of_predicate: next_object.is_none(),
                ..flags
            };
            self.write_object(w, graph, object, max_len, flags)?;
        }
        Ok(())
    }

    fn write_sub_graph<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        subject: &SubjectNode,
        flags: WriterStatusFlags,
    ) -> rdftk_core::error::Result<()> {
        self.write_subject(w, graph, subject, flags)?;
        self.write_predicates_of_subject(w, graph, subject, flags)
    }

    fn write_predicates_of_subject<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        subject: &SubjectNode,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        let all_predicates = Vec::from_iter(graph.predicates_for(subject));
        let mut count = 0;
        let total_number = all_predicates.len();
        let max_len = 1 + self.max_len_predicates(graph, &all_predicates)?;

        // let max_len = all_predicates.iter().(|)
        //     .fold(std::u16::MIN, |a,b| a.max(b.borrow().));
        for (group, ref mut preds) in PredicateGroupOrdering::group_predicates(&all_predicates) {
            preds.sort_by_cached_key(|iri| self.compress_iri(graph, iri).unwrap());
            for predicate in preds {
                count += 1;
                let flags = WriterStatusFlags {
                    is_last_of_subject: count == total_number,
                    ..flags
                };
                self.write_predicate_object(w, graph, group, subject, predicate, max_len, flags)?;
            }
        }
        writeln!(w, " .")?;

        Ok(())
    }

    /// Deal with a nested blank node.
    fn write_nested_blank_node<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        object: &ObjectNode,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        write!(w, "[")?;
        let inner_subject = object.to_subject().unwrap();
        self.write_sub_graph(w, graph, &inner_subject, flags)?;
        self.wrote_blank(&inner_subject);
        self.new_line(w, flags)?;
        write!(w, "]")?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > Ordering
// ------------------------------------------------------------------------------------------------

impl PredicateGroupOrdering {
    fn group_predicates<'a>(predicates: &[&'a Iri]) -> Vec<(PredicateGroupOrdering, Vec<&'a Iri>)> {
        let mut result = predicates
            .iter()
            .chunk_by(Self::group_predicate)
            .into_iter()
            .map(|(triple_type, group)| (triple_type, group.cloned().collect()))
            .collect::<Vec<(PredicateGroupOrdering, Vec<&Iri>)>>();
        result.sort_by_key(|a| a.0);
        result
    }

    fn group_predicate(predicate: &&&Iri) -> PredicateGroupOrdering {
        match predicate.to_string().as_str() {
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" => PredicateGroupOrdering::Type,
            "http://www.w3.org/2000/01/rdf-schema#label" => PredicateGroupOrdering::Label,
            "http://xmlns.com/foaf/0.1/name" => PredicateGroupOrdering::Label,
            "http://purl.org/dc/elements/1.1/title" => PredicateGroupOrdering::Label,
            "http://www.w3.org/2000/01/rdf-schema#comment" => PredicateGroupOrdering::Comment,
            "http://purl.org/dc/elements/1.1/description" => PredicateGroupOrdering::Comment,
            _ => PredicateGroupOrdering::Other,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::PredicateGroupOrdering;
    use super::PredicateGroupOrdering::*;

    #[test]
    fn test_order() {
        let mut v: Vec<PredicateGroupOrdering> = vec![Comment, Label, Type, Other];
        v.sort();
        let sorted = format!("{:?}", v);
        assert_eq!(sorted, "[Type, Label, Comment, Other]");
    }
}
