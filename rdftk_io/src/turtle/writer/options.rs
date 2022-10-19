use rdftk_iri::IRIRef;

#[derive(Debug, Clone)]
pub struct TurtleOptions {
    pub id_base: Option<IRIRef>,
    pub nest_blank_nodes: bool,
    pub use_sparql_style: bool,
    /// Use the same formatting style as used by the LNKD.tech editor plugin
    /// for the IntelliJ IDEs like Idea and CLion
    pub use_intellij_style: bool,
    /// Some prefer to show the "a <type>" statement on the same line as
    /// the subject IRI.
    pub place_type_on_subject_line: bool,
    /// If provided, any IRI that's written to Turtle that starts with the given
    /// IRI will be written to Turtle as if it's part of the base namespace.
    pub convert_to_id_base: Option<IRIRef>,
    /// If provided, any IRI that's written to Turtle that starts with the given
    /// IRI will be converted with the provided second base IRI.
    pub convert_base: Vec<(IRIRef, IRIRef)>,
    pub indent_width: u16,
}

impl Default for TurtleOptions {
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

impl TurtleOptions {
    pub fn default_with_base(base: &IRIRef) -> Self {
        Self::default().with_id_base(Some(base))
    }

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

    pub fn new_with_intellij_style_with_type_on_subject_line() -> Self {
        Self {
            use_intellij_style: true,
            place_type_on_subject_line: true,
            indent_width: 4,
            ..Default::default()
        }
    }

    /// Return a new instance of the given `TurtleOptions` where the `id_base` is set to the given
    /// IRI which will instruct the `TurtleWriter` to generate a `@base <id_base>` or `BASE <id_base>`
    /// statement at the top of the file.
    pub fn with_id_base<'a>(mut self, id_base: Option<&'a IRIRef>) -> Self {
        self.id_base = id_base.cloned();
        self
    }

    pub fn with_conversion_to_id_base(mut self, from_base: Option<&IRIRef>) -> Self {
        self.convert_to_id_base = from_base.cloned();
        self
    }

    pub fn with_iri_conversion(mut self, from_base: IRIRef, to_base: IRIRef) -> Self {
        self.convert_base.push((from_base, to_base));
        self
    }

    pub fn with_indent_width(mut self, width: u16) -> Self {
        self.indent_width = width;
        self
    }
}
