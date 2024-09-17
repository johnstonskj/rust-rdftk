use rdftk_iri::IriRef;

#[derive(Clone, Debug)]
pub struct TurtleOptions {
    id_base: Option<IriRef>,
    nest_blank_nodes: bool,
    use_sparql_style: bool,
    use_intellij_style: bool,
    place_type_on_subject_line: bool,
    convert_to_id_base: Option<IriRef>,
    convert_base: Vec<(IriRef, IriRef)>,
    indent_width: usize,
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
    /// Return a new instance of the given `TurtleOptions` where the `id_base` is set to the given
    /// Iri which will instruct the `TurtleWriter` to generate a `@base <id_base>` or `BASE <id_base>`
    /// statement at the top of the file.
    pub fn with_id_base(self, id_base: IriRef) -> Self {
        Self {
            id_base: Some(id_base.clone()),
            ..self
        }
    }

    /// Set default options to make the generated Turtle RDF look like it's formatted
    /// by the LNKD.tech plugin that is used in the IntelliJ family of editors such as
    /// Idea and CLion.
    /// This would allow you to load RDF from a git clone and write it back to disk
    /// without causing unnecessary git-diff detectable changes.
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

    pub fn id_base(&self) -> Option<&IriRef> {
        self.id_base.as_ref()
    }

    pub fn set_id_base(&mut self, id_base: IriRef) {
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

    /// Use the same formatting style as used by the LNKD.tech editor plugin
    /// for the IntelliJ IDEs like Idea and CLion
    pub fn use_intellij_style(&self) -> bool {
        self.use_intellij_style
    }

    pub fn set_use_intellij_style(&mut self, use_intellij_style: bool) {
        self.use_intellij_style = use_intellij_style;
    }

    /// Some prefer to show the "a <type>" statement on the same line as
    /// the subject Iri.
    pub fn place_type_on_subject_line(&self) -> bool {
        self.place_type_on_subject_line
    }

    pub fn set_place_type_on_subject_line(&mut self, place_type_on_subject_line: bool) {
        self.place_type_on_subject_line = place_type_on_subject_line;
    }

    /// If provided, any Iri that's written to Turtle that starts with the given
    /// Iri will be written to Turtle as if it's part of the base namespace.
    pub fn convert_to_id_base(&self) -> Option<&IriRef> {
        self.convert_to_id_base.as_ref()
    }

    pub fn set_convert_to_id_base(&mut self, convert_to_id_base: IriRef) {
        self.convert_to_id_base = Some(convert_to_id_base);
    }

    pub fn unset_convert_to_id_base(&mut self) {
        self.convert_to_id_base = None;
    }

    /// If provided, any Iri that's written to Turtle that starts with the given
    /// Iri will be converted with the provided second base Iri.
    pub fn convert_base(&self) -> &Vec<(IriRef, IriRef)> {
        &self.convert_base
    }

    pub fn indent_width(&self) -> usize {
        self.indent_width
    }

    pub fn set_indent_width(&mut self, indent_width: usize) {
        self.indent_width = indent_width;
    }
}
