use crate::{common::indenter::Indenter, GraphWriter};
use itertools::Itertools;
use objio::{impl_has_options, HasOptions, ObjectWriter};
use rdftk_core::{
    error::{Error, Result},
    model::{
        graph::Graph,
        literal::{DataType, Literal},
        statement::{Collection, ObjectNode, Statement, SubjectNode},
    },
};
use rdftk_iri::Iri;
use rdftk_names::{dc::elements, foaf, owl, rdf, rdfs, skos};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    io::Write,
    str::FromStr,
    sync::LazyLock,
};
use tracing::trace;

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
    outdent_blank_node_end: bool,
    outdent_collection_end: bool,
    use_rdf_type_a: bool,
    use_sparql_style: bool,
    use_intellij_style: bool,
    place_type_on_subject_line: bool,
    convert_to_id_base: Option<Iri>,
    convert_base: Vec<(Iri, Iri)>,
    indent_width: usize,
    predicate_padding: bool,
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
    blanks_to_write: HashSet<SubjectNode>,
}

const DECL_BASE_TTL: &str = "@base";
const DECL_BASE_SPARQL: &str = "BASE";
const DECL_PREFIX_TTL: &str = "@prefix";
const DECL_PREFIX_SPARQL: &str = "PREFIX";
const NAME_SEPARATOR: &str = ":";
const IRI_START: &str = "<";
const IRI_END: &str = ">";
const BLANK_NODE_PREFIX: &str = "_";
const BLANK_NODE_START: &str = "[";
const BLANK_NODE_END: &str = "]";
const COLLECTION_START: &str = "(";
const COLLECTION_END: &str = ")";
const LANGUAGE_PREFIX: &str = "@";
const DATATYPE_PREFIX: &str = "^^";
const PREDICATE_SEPARATOR: &str = " ;";
const OBJECT_SEPARATOR: &str = ",";
const SPACE_SEPARATOR: &str = " ";
const END_OF_STATEMENT: &str = " .";
const END_OF_LINE: &str = "\n";
const RDF_TYPE_A: &str = "a";

// ------------------------------------------------------------------------------------------------
// Implementations > Options
// ------------------------------------------------------------------------------------------------

impl Default for TurtleWriterOptions {
    fn default() -> Self {
        Self {
            id_base: None,
            nest_blank_nodes: true,
            outdent_blank_node_end: false,
            outdent_collection_end: false,
            use_rdf_type_a: false,
            use_sparql_style: false,
            use_intellij_style: false,
            place_type_on_subject_line: false,
            convert_to_id_base: None,
            convert_base: Vec::new(),
            indent_width: 4,
            predicate_padding: false,
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

    pub fn with_predicate_padding(self, predicate_padding: bool) -> Self {
        Self {
            predicate_padding,
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

    pub fn predicate_padding(&self) -> bool {
        self.predicate_padding
    }

    pub fn set_predicate_padding(&mut self, predicate_padding: bool) {
        self.predicate_padding = predicate_padding;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > Writer
// ------------------------------------------------------------------------------------------------

impl_has_options!(TurtleWriter, TurtleWriterOptions);

impl TurtleWriter {
    pub fn with_options(self, options: TurtleWriterOptions) -> Self {
        let mut self_mut = self;
        self_mut.set_options(options);
        self_mut
    }
}

impl ObjectWriter<Graph> for TurtleWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &Graph) -> Result<()>
    where
        W: Write,
    {
        let (nested, plain): (Vec<_>, Vec<_>) =
            graph.statements().partition(|stmt| stmt.is_nested());

        let mut reified_statements: Vec<Statement> = nested
            .into_iter()
            .map(|stmt| {
                let (subject, mut new) = stmt.reify().unwrap();
                new.push(Statement::new(
                    stmt.subject(),
                    stmt.predicate().clone(),
                    subject.to_object(),
                ));
                new
            })
            .flatten()
            .collect();
        plain
            .into_iter()
            .for_each(|s| reified_statements.push(s.clone()));

        let mut denested_graph = Graph::from(reified_statements);
        denested_graph.set_prefix_mappings(graph.prefix_mappings().clone());
        self.write_turtle_doc(w, &denested_graph)
    }
}

impl GraphWriter for TurtleWriter {}

impl TurtleWriter {
    // ---------------------------------------------------------------------------------------------
    // Formatting
    // ---------------------------------------------------------------------------------------------

    #[inline(always)]
    fn indent(&self) {
        let context = self.context.borrow();
        context.indenter.indent();
    }

    #[inline(always)]
    fn outdent(&self) {
        let context = self.context.borrow();
        context.indenter.outdent();
    }

    #[inline(always)]
    fn new_line<W: Write>(&self, w: &mut W, flags: WriterStatusFlags) -> Result<()> {
        trace!(name: "new_line", ?flags);
        if flags.is_being_sorted {
            Ok(write!(w, "{SPACE_SEPARATOR}")?)
        } else {
            let context = self.context.borrow();
            write!(w, "{END_OF_LINE}{}", context.indenter)?;
            Ok(())
        }
    }

    #[inline(always)]
    fn write_padded<W: Write>(&self, w: &mut W, value: &str, max_len: usize) -> Result<()> {
        trace!(name: "write_padded", value, max_len);
        Ok(if max_len == 0 {
            write!(w, "{}{}", value, SPACE_SEPARATOR)
        } else {
            write!(w, "{:<max_len$}", value)
        }?)
    }

    #[inline(always)]
    fn write_padding<W: Write>(&self, w: &mut W, max_len: usize) -> Result<()> {
        trace!(name: "write_padding", max_len);
        Ok(if max_len == 0 {
            write!(w, " ")
        } else {
            write!(w, "{:max_len$}", SPACE_SEPARATOR)
        }?)
    }

    // ---------------------------------------------------------------------------------------------
    // State Management
    // ---------------------------------------------------------------------------------------------

    fn sorted_subjects(&self, graph: &Graph) -> Vec<SubjectNode> {
        trace!("sorted_subjects");
        graph
            .node_subjects()
            .into_iter()
            .sorted()
            .cloned()
            .collect::<Vec<SubjectNode>>()
    }

    /// Calculate the longest predicate name and use as the width of the current indentation.
    fn max_len_predicates(&self, graph: &Graph, predicates: &[&Iri]) -> Result<usize> {
        trace!("max_len_predicates");
        let all_predicates_as_strings = predicates
            .iter()
            .map(|iri| self.compress_iri(graph, iri))
            .collect::<Result<Vec<String>>>()?
            .iter()
            .fold(0, |a, b| a.max(b.len()));
        Ok(all_predicates_as_strings)
    }

    fn object_sort_key(&self, graph: &Graph, object: &ObjectNode) -> Result<String> {
        trace!("object_sort_key");
        let mut buffer = Vec::<u8>::new();
        let new_writer = Self::default().with_options(self.options.clone());
        let flags = WriterStatusFlags {
            is_being_sorted: true,
            ..Default::default()
        };
        new_writer.write_object(&mut buffer, graph, object, flags)?;
        Ok(String::from_utf8(buffer)?)
    }

    // ---------------------------------------------------------------------------------------------
    // Grammar
    // ---------------------------------------------------------------------------------------------

    ///
    /// ```text
    /// [1]  turtleDoc  ::= statement*
    /// [2]  statement  ::= directive | triples '.'
    /// [3]  directive  ::= prefixID | base | sparqlPrefix | sparqlBase
    ///```
    ///
    fn write_turtle_doc<W: Write>(&self, w: &mut W, graph: &Graph) -> Result<()> {
        trace!("write_turtle_doc");
        {
            let mut context_mut = self.context.borrow_mut();
            context_mut.indenter =
                Indenter::default().with_default_indent_width(self.options.indent_width());
            // Initialize with all the blank nodes in the graph
            context_mut.blanks_to_write = graph
                .blank_node_subjects()
                .iter()
                .map(|s| {
                    let s = *s;
                    s.clone()
                })
                .collect();
        }

        // [3] directives
        self.write_base_iri(w)?;
        self.write_prefixes(w, graph)?;

        // [2] ... triples
        let flags = WriterStatusFlags::default();
        self.write_triples(w, graph, flags)?;
        Ok(())
    }

    ///
    /// ```text
    /// [5]   base        ::= '@base' IRIREF '.'
    /// [5s]  sparqlBase  ::= "BASE" IRIREF
    /// ```
    ///
    fn write_base_iri<W: Write>(&self, w: &mut W) -> Result<()> {
        trace!("write_base_iri");
        if let Some(base) = &self.options.id_base() {
            let (decl, eos) =
                if self.options.use_sparql_style() && !self.options.use_intellij_style() {
                    (DECL_BASE_SPARQL, "")
                } else {
                    (DECL_BASE_TTL, END_OF_STATEMENT)
                };
            writeln!(
                w,
                "{decl} {IRI_START}{}{IRI_END}{eos}",
                base.to_string().as_str()
            )?;
            if !self.options.use_intellij_style() {
                writeln!(w)?;
            }
        }
        Ok(())
    }

    ///
    /// ```text
    /// [4]   prefixID      ::= '@prefix' PNAME_NS IRIREF '.'
    /// [6s]  sparqlPrefix  ::= "PREFIX" PNAME_NS IRIREF
    /// ```
    ///
    fn write_prefixes<W: Write>(&self, w: &mut W, graph: &Graph) -> Result<()> {
        trace!("write_prefixes");
        let mappings = graph.prefix_mappings();
        if !mappings.is_empty() {
            for (prefix, namespace) in mappings.mappings().sorted() {
                let prefix = prefix.as_ref().map(|n| n.as_ref()).unwrap_or("");
                let mut namespace_str = namespace.to_string();
                // If we have any base Iri conversions to do for any of the
                // namespaces, then do it now:
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
                trace!("write_prefixes {prefix}: {IRI_START}{namespace_str}{IRI_END}");
                let (decl, eos) =
                    if self.options.use_sparql_style() && !self.options.use_intellij_style() {
                        (DECL_PREFIX_SPARQL, "")
                    } else {
                        (DECL_PREFIX_TTL, END_OF_STATEMENT)
                    };
                writeln!(
                    w,
                    "{decl} {prefix}{NAME_SEPARATOR} {IRI_START}{namespace_str}{IRI_END}{eos}"
                )?;
            }
            writeln!(w)?;
        }
        Ok(())
    }

    ///
    /// ```text
    /// [6]   triples  ::= subject predicateObjectList
    ///                  | blankNodePropertyList predicateObjectList?
    /// ```
    ///
    fn write_triples<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        trace!(name: "write_triples", ?flags);
        for subject in self.sorted_subjects(graph) {
            self.write_subject(w, graph, &subject, flags)?;
            self.write_predicate_object_list(w, graph, &subject, flags)?;
            writeln!(w)?;
        }

        // The given cursor object collects all the blank-node objects that have not
        // been written to the turtle file yet but have been referred to during
        // the call to `write_normal_subjects` above. Now process those
        // unwritten blank nodes and add them to the end of the file.
        let context = self.context.borrow();
        // Iterate over all blank nodes
        for subject in context.blanks_to_write.iter() {
            context.indenter.reset_depth();
            self.write_subject(w, graph, subject, flags)?;
            self.write_predicate_object_list(w, graph, subject, flags)?;
            writeln!(w)?;
        }
        Ok(())
    }

    ///
    /// ```text
    /// [10]  subject  ::= iri | BlankNode | collection
    /// ```
    ///
    fn write_subject<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        subject: &SubjectNode,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        trace!(name: "write_subject", ?subject, ?flags);
        let at_start_of_line = self.context.borrow().indenter.is_not_indented();
        match (subject, at_start_of_line) {
            (SubjectNode::Blank(blank), true) => {
                let initial = if flags.is_being_sorted {
                    SPACE_SEPARATOR
                } else {
                    END_OF_LINE
                };
                write!(w, "{initial}{BLANK_NODE_PREFIX}{NAME_SEPARATOR}{blank}")?;
            }
            (SubjectNode::Resource(_), _) => {
                self.write_iri(w, graph, subject.as_resource().unwrap())?;
            }
            (SubjectNode::Statement(_), _) => {
                unreachable!("RDF-* Statements are not supported in Turtle representation")
            }
            _ => {}
        }
        self.indent();
        Ok(())
    }

    ///
    /// ```text
    /// [7]  predicateObjectList  ::= verb objectList (';' (verb objectList)?)*
    /// ```
    ///
    fn write_predicate_object_list<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        subject: &SubjectNode,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        trace!(name: "write_predicate_object_list", ?subject, ?flags);
        let all_predicates = Vec::from_iter(graph.predicates_for(subject));
        let mut count = 0;
        let total_number = all_predicates.len();
        let max_len = if self.options.predicate_padding {
            0
        } else {
            1 + self.max_len_predicates(graph, &all_predicates)?
        };

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

        Ok(())
    }

    ///
    /// ```text
    /// [9]  verb  ::= predicate | 'a'
    /// ```
    ///
    fn write_verb<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        group: PredicateGroupOrdering,
        predicate: &Iri,
        max_len: usize,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        trace!(name: "write_verb", ?predicate, ?group, max_len, ?flags);
        // Special treatment for `rdf:type`; show it in turtle as just "a"
        if group == PredicateGroupOrdering::Type {
            let type_verb = if self.options.use_rdf_type_a {
                RDF_TYPE_A.to_string()
            } else {
                self.compress_iri(graph, rdf::a_type())?
            };
            return if self.options.place_type_on_subject_line() {
                Ok(write!(w, " {type_verb} ")?)
            } else {
                self.new_line(w, flags)?;
                self.write_padded(w, &type_verb, max_len)
            };
        }
        // Otherwise, go to the next line and write it as a normal predicate-Iri
        self.new_line(w, flags)?;
        let pred = self.compress_iri(graph, predicate)?;
        self.write_padded(w, pred.as_str(), max_len)
    }

    ///
    /// ```text
    /// [135s]  iri           ::= IRIREF | PrefixedName
    /// [136s]  PrefixedName  ::= PNAME_LN | PNAME_NS
    /// ```
    ///
    #[inline(always)]
    fn write_iri<W: Write>(&self, w: &mut W, graph: &Graph, iri: &Iri) -> Result<()> {
        trace!(name: "write_iri", ?iri);
        Ok(write!(w, "{}", self.compress_iri(graph, iri)?)?)
    }

    /// Compress any Iri to its "QName" given the supplied set of prefixes and
    /// their namespace Iris. If we're encountering an Iri whose prefix
    /// equals the given (optional) `convert_to_base` Iri then write it to
    /// Turtle as if it's an Iri with the default base.
    fn compress_iri(&self, graph: &Graph, iri: &Iri) -> Result<String> {
        trace!(name: "compress_iri", ?iri);
        let mut iri_str = iri.to_string();
        if let Some(id_base) = &self.options.id_base() {
            if let Some(ref convert_to_id_base) = self.options.convert_to_id_base() {
                let target_id_base = convert_to_id_base.to_string();
                if iri_str.starts_with(target_id_base.as_str()) {
                    return Ok(format!(
                        "{IRI_START}{}{IRI_END}",
                        &iri_str[target_id_base.len()..]
                    ));
                }
            }
            let id_base_str = id_base.to_string();
            if iri_str.starts_with(id_base_str.as_str()) {
                return Ok(format!(
                    "{IRI_START}{}{IRI_END}",
                    &iri_str[id_base_str.len()..]
                ));
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
            None => format!("{IRI_START}{iri}{IRI_END}"),
            Some(_qname) => format!("{_qname}"),
        })
    }

    ///
    /// ```text
    /// [13]    literal         ::= RDFLiteral | NumericLiteral | BooleanLiteral
    /// [16]    NumericLiteral  ::= INTEGER | DECIMAL | DOUBLE
    /// [128s]  RDFLiteral      ::= String (LANGTAG | '^^' iri)?
    /// [133s]  BooleanLiteral  ::= 'true' | 'false'
    /// [17]    String          ::= STRING_LITERAL_QUOTE | STRING_LITERAL_SINGLE_QUOTE
    ///                           | STRING_LITERAL_LONG_SINGLE_QUOTE
    ///                           | STRING_LITERAL_LONG_QUOTE
    /// ```
    //
    fn write_literal<W: Write>(&self, w: &mut W, graph: &Graph, literal: &Literal) -> Result<()> {
        trace!(name: "write_literal", ?literal);
        Ok(match literal.data_type() {
            Some(DataType::Iri) => {
                let iri = Iri::parse(literal.lexical_form())?;
                self.write_iri(w, graph, &iri)?
            }
            Some(DataType::Boolean)
            | Some(DataType::Long)
            | Some(DataType::Int)
            | Some(DataType::Short)
            | Some(DataType::Byte)
            | Some(DataType::UnsignedLong)
            | Some(DataType::UnsignedInt)
            | Some(DataType::UnsignedShort)
            | Some(DataType::UnsignedByte)
            | Some(DataType::Float)
            | Some(DataType::Double)
            | Some(DataType::Decimal) => write!(w, "{}", literal.lexical_form())?,
            _ => {
                write!(w, "{:?}", literal.lexical_form())?;
                match (literal.data_type(), literal.language()) {
                    (Some(data_type), None) => {
                        write!(w, "{DATATYPE_PREFIX}")?;
                        let iri = data_type.as_iri();
                        self.write_iri(w, graph, iri)?;
                    }
                    (None, Some(language)) => write!(w, "{LANGUAGE_PREFIX}{}", language)?,
                    _ => (),
                }
            }
        })
    }

    fn write_predicate_object_object<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        object: &ObjectNode,
        max_len: usize,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        trace!(name: "write_predicate_object_object", ?object, max_len, ?flags);
        self.write_object(w, graph, object, flags)?;
        if flags.is_last_of_predicate {
            if flags.is_last_of_subject {
                self.outdent();
            }
            if self.context.borrow().indenter.is_not_indented() {
                write!(w, "{END_OF_STATEMENT}")?;
                self.new_line(w, flags)?;
            } else if !flags.is_last_of_subject {
                write!(w, "{PREDICATE_SEPARATOR}")?;
            }
        } else {
            write!(w, "{OBJECT_SEPARATOR}")?;
            if !flags.is_next_object_blank {
                self.indent();
                self.new_line(w, flags)?;
                if max_len > 0 {
                    self.write_padding(w, max_len)?;
                }
                self.outdent();
            }
        }
        Ok(())
    }

    ///
    /// ```text
    /// [12]  object  ::= iri | BlankNode | collection | blankNodePropertyList | literal
    /// ````
    ///
    fn write_object<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        object: &ObjectNode,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        trace!(name: "write_object", ?object, ?flags);
        match &object {
            ObjectNode::Blank(blank) => {
                if self.options.nest_blank_nodes()
                    && graph.contains_subject(&object.to_subject().unwrap())
                {
                    self.write_blank_node_property_list(w, graph, object, flags)?;
                } else if self.options.nest_blank_nodes()
                    && !graph.contains_subject(&object.to_subject().unwrap())
                {
                    write!(w, "{BLANK_NODE_START}{BLANK_NODE_END}")?;
                } else {
                    write!(w, "{BLANK_NODE_PREFIX}{NAME_SEPARATOR}{blank}",)?;
                }
            }
            ObjectNode::Resource(iri) => {
                self.write_iri(w, graph, iri)?;
            }
            ObjectNode::Literal(value) => {
                self.write_literal(w, graph, value)?;
            }
            ObjectNode::Collection(lst) => {
                self.write_collection(w, graph, lst, flags)?;
            }
            ObjectNode::Statement(_) => {
                unreachable!("RDF-* Statements are not supported in Turtle representation")
            }
        }
        Ok(())
    }

    ///
    /// ```text
    /// [15]  collection  ::=	'(' object* ')'
    /// ```
    ///
    #[inline(always)]
    fn write_collection<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        collection: &Collection,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        trace!(name: "write_collection", ?collection, ?flags);
        if !collection.is_empty() {
            self.indent();
            write!(w, "{COLLECTION_START}")?;
            self.new_line(w, flags)?;
            for (idx, object) in collection.iter().enumerate() {
                self.write_object(w, graph, object, flags)?;
                if idx < collection.len() - 1 {
                    write!(w, "{OBJECT_SEPARATOR}{SPACE_SEPARATOR}")?;
                }
            }
            if self.options.outdent_collection_end {
                self.outdent();
            }
            self.new_line(w, flags)?;
            write!(w, "{COLLECTION_END}")?;
            if !self.options.outdent_collection_end {
                self.outdent();
            }
        } else {
            write!(w, "{COLLECTION_START}{COLLECTION_END}")?;
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
        trace!(
            name: "write_predicate_object",
            ?subject, ?predicate, ?group, max_len, ?flags
        );
        // First, write the predicate
        self.write_verb(w, graph, group, predicate, max_len, flags)?;

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
            self.write_predicate_object_object(w, graph, object, max_len, flags)?;
        }
        Ok(())
    }
    ///
    /// ```text
    /// [14]  blankNodePropertyList  ::= '[' predicateObjectList ']'
    /// ```
    ///
    fn write_blank_node_property_list<W: Write>(
        &self,
        w: &mut W,
        graph: &Graph,
        object: &ObjectNode,
        flags: WriterStatusFlags,
    ) -> Result<()> {
        trace!(name: "write_blank_node_property_list", ?object, ?flags);
        self.indent();
        write!(w, "{BLANK_NODE_START}")?;
        let inner_subject = object.to_subject().unwrap();
        self.write_predicate_object_list(w, graph, &inner_subject, flags)?;
        self.new_line(w, flags)?;
        write!(w, "{BLANK_NODE_END}")?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > Ordering
// ------------------------------------------------------------------------------------------------

static PREDICATE_GROUP_MAP: LazyLock<BTreeMap<Iri, PredicateGroupOrdering>> =
    LazyLock::new(PredicateGroupOrdering::new_mapping);

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
        PREDICATE_GROUP_MAP
            .get(predicate)
            .map(|v| *v)
            .unwrap_or_else(|| PredicateGroupOrdering::Other)
    }

    fn new_mapping() -> BTreeMap<Iri, Self> {
        vec![
            (rdf::a_type().clone(), Self::Type),
            (rdfs::subclass_of().clone(), Self::Type),
            (rdfs::subproperty_of().clone(), Self::Type),
            (owl::equivalent_class().clone(), Self::Type),
            (rdfs::label().clone(), Self::Label),
            (skos::pref_label().clone(), Self::Label),
            (skos::alt_label().clone(), Self::Label),
            (skos::hidden_label().clone(), Self::Label),
            (foaf::name().clone(), Self::Label),
            (elements::title().clone(), Self::Label),
            (rdfs::comment().clone(), Self::Comment),
            (elements::description().clone(), Self::Comment),
            (skos::definition().clone(), Self::Comment),
            (skos::note().clone(), Self::Comment),
            (skos::scope_note().clone(), Self::Comment),
            (skos::editorial_note().clone(), Self::Comment),
        ]
        .into_iter()
        .collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::PredicateGroupOrdering::*;
    use super::{PredicateGroupOrdering, TurtleWriter};
    use objio::ObjectWriter;
    use rdftk_core::model::graph::Graph;
    use rdftk_core::model::literal::Literal;
    use rdftk_core::model::statement::{BlankNode, Collection, ObjectNode, Statement};
    use rdftk_names::rdfs;
    use std::str::FromStr;

    #[test]
    fn test_order() {
        let mut v: Vec<PredicateGroupOrdering> = vec![Comment, Label, Type, Other];
        v.sort();
        let sorted = format!("{:?}", v);
        assert_eq!(sorted, "[Type, Label, Comment, Other]");
    }

    #[test]
    fn test_blank_collection_object() {
        let list: Vec<ObjectNode> = vec![
            BlankNode::from_str("aa").unwrap().into(),
            BlankNode::from_str("bb").unwrap().into(),
            BlankNode::from_str("cc").unwrap().into(),
        ];
        let collection: Collection = list.into();
        let statement = Statement::new(BlankNode::generate(), rdfs::label().clone(), collection);
        let graph = Graph::from(vec![statement]);

        let writer = TurtleWriter::default();
        writer.write(&mut std::io::stdout(), &graph).unwrap();
    }

    #[test]
    fn test_literal_collection_object() {
        let list: Vec<ObjectNode> = vec![
            Literal::plain("aa").into(),
            Literal::plain("bb").into(),
            Literal::plain("cc").into(),
        ];
        let collection: Collection = list.into();
        let statement = Statement::new(BlankNode::generate(), rdfs::label().clone(), collection);
        let graph = Graph::from(vec![statement]);

        let writer = TurtleWriter::default();
        writer.write(&mut std::io::stdout(), &graph).unwrap();
    }
}
