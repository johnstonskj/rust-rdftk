use rdftk_iri::IRIRef;

#[derive(Debug, Clone)]
pub struct TurtleOptions {
    pub base: Option<String>,
    pub nest_blank_nodes: bool,
    pub use_sparql_style: bool,
    /// Use the same formatting style as used by the LNKD.tech editor plugin
    /// for the IntelliJ IDEs like Idea and CLion
    pub use_intellij_style: bool,
    /// Some prefer to show the "a <type>" statement on the same line as
    /// the subject IRI.
    pub place_type_on_subject_line: bool,
    /// If provided, any IRI that's written to Turtle that starts with the given
    /// string will be written to Turtle as if it's part of the base namespace.
    pub convert_to_base: Option<String>,
    pub indent_width: u16,
}

impl Default for TurtleOptions {
    fn default() -> Self {
        Self {
            base: None,
            nest_blank_nodes: true,
            use_sparql_style: false,
            use_intellij_style: false,
            place_type_on_subject_line: false,
            convert_to_base: None,
            indent_width: 2,
        }
    }
}

impl TurtleOptions {
    pub fn default_with_base(base: IRIRef) -> Self {
        Self::default().with_base(base)
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

    pub fn with_base(mut self, base: IRIRef) -> Self {
        self.base = Some(base.to_string());
        self
    }

    pub fn with_indent_width(mut self, width: u16) -> Self {
        self.indent_width = width;
        self
    }
}
