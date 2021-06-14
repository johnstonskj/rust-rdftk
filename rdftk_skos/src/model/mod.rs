/*!
A simple model for constructing SKOS thesauri. This is not a complete API in
that it's extensibility with OWL is limited.
*/

use crate::ns;
use rdftk_core::model::graph::mapping::PrefixMappingRef;
use rdftk_core::model::graph::{GraphFactoryRef, GraphRef};
use rdftk_core::model::literal::{LanguageTag, LiteralFactoryRef};
use rdftk_core::model::statement::{
    ObjectNodeRef, StatementFactoryRef, StatementList, StatementRef, SubjectNodeRef,
};
use rdftk_iri::IRIRef;
use rdftk_names::{dc, owl};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Labeled {
    fn add_label(&mut self, label: Label);

    fn has_labels(&self) -> bool;

    fn labels(&self) -> &Vec<Label>;

    // --------------------------------------------------------------------------------------------

    fn add_preferred_label(&mut self, text: &str, language: &str) {
        self.add_label(Label::preferred(text, language))
    }

    fn add_alternative_label(&mut self, text: &str, language: &str) {
        self.add_label(Label::alternative(text, language))
    }

    fn add_hidden_label(&mut self, text: &str, language: &str) {
        self.add_label(Label::hidden(text, language))
    }

    fn get_preferred_label_for(&self, language: &Option<LanguageTag>) -> String;
}

pub trait Propertied {
    fn add_property(&mut self, property: LiteralProperty);

    fn has_property(&self, predicate: &IRIRef) -> bool {
        self.properties()
            .iter()
            .any(|property| property.predicate() == predicate)
    }

    fn has_properties(&self) -> bool {
        !self.properties().is_empty()
    }

    fn properties(&self) -> &Vec<LiteralProperty>;

    // --------------------------------------------------------------------------------------------

    fn define(&mut self, text: &str, language: &str) -> &mut Self {
        self.add_property(LiteralProperty::definition_with(text, language));
        self
    }

    fn notation(&mut self, notation: &str) -> &mut Self {
        self.add_property(LiteralProperty::notation(notation));
        self
    }

    fn copyright(&mut self, publisher: &str, rights: &str) -> &mut Self {
        self.add_property(LiteralProperty::publisher(publisher));
        self.add_property(LiteralProperty::rights(rights));
        self
    }
}

pub trait Resource: Labeled + Propertied {
    fn uri(&self) -> &IRIRef;
}

pub trait ToStatements {
    fn to_statements(
        &self,
        in_scheme: Option<&ObjectNodeRef>,
        statements: &StatementFactoryRef,
        literals: &LiteralFactoryRef,
    ) -> StatementList;
}

pub trait ToStatement {
    fn to_statement(
        &self,
        subject: &SubjectNodeRef,
        statements: &StatementFactoryRef,
        literals: &LiteralFactoryRef,
    ) -> StatementRef;
}

pub trait ToUri {
    fn to_uri(&self) -> IRIRef;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn to_rdf_graph(
    scheme: &Scheme,
    default_namespace: Option<IRIRef>,
    factory: &GraphFactoryRef,
) -> GraphRef {
    let ns_mappings = standard_mappings(factory);
    if let Some(default_namespace) = default_namespace {
        let mut ns_mappings = ns_mappings.borrow_mut();
        let _ = ns_mappings.set_default_namespace(default_namespace);
    }
    to_rdf_graph_with_mappings(scheme, ns_mappings, factory)
}

pub fn to_rdf_graph_with_mappings(
    scheme: &Scheme,
    ns_mappings: PrefixMappingRef,
    factory: &GraphFactoryRef,
) -> GraphRef {
    let graph = factory.graph();
    {
        let mut graph = graph.borrow_mut();
        let _ = graph.set_prefix_mappings(ns_mappings);

        for statement in
            scheme.to_statements(None, &graph.statement_factory(), &graph.literal_factory())
        {
            graph.insert(statement.into());
        }
    }
    graph
}

pub fn from_rdf_graph<'a>(_graph: &GraphRef) -> Vec<Scheme> {
    // let schemes = Default::default();
    // let graph = graph.borrow();
    // let scheme_subjects: Vec<&SubjectNodeRef> = graph
    //     .statements()
    //     .filter_map(|st| {
    //         if st.predicate() == rdf::a_type() && object_eq_iri(st.object(), ns::concept_scheme()) {
    //             Some(st.subject())
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    //for subject in scheme_subjects {}
    todo!()
}

pub fn standard_mappings(factory: &GraphFactoryRef) -> PrefixMappingRef {
    let mappings = factory.mapping_factory().common();
    {
        let mut mut_mappings = mappings.borrow_mut();
        let _ = mut_mappings.insert(ns::default_prefix(), ns::namespace_iri().clone());
        let _ = mut_mappings.insert(ns::xl::default_prefix(), ns::xl::namespace_iri().clone());
        let _ = mut_mappings.insert(ns::iso::default_prefix(), ns::iso::namespace_iri().clone());
        let _ = mut_mappings.insert(
            ns::term_status::default_prefix(),
            ns::term_status::namespace_iri().clone(),
        );
        let _ = mut_mappings.insert(
            dc::terms::default_prefix(),
            dc::terms::namespace_iri().clone(),
        );
        let _ = mut_mappings.insert(owl::default_prefix(), owl::namespace_iri().clone());
    }
    mappings
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[allow(dead_code)]
fn object_eq_iri(object: &ObjectNodeRef, iri: &IRIRef) -> bool {
    if let Some(lhs) = object.as_iri() {
        lhs == iri
    } else {
        false
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod scheme;
pub use scheme::Scheme;

pub mod concept;
pub use concept::Concept;

pub mod collection;
pub use collection::Collection;

pub mod properties;
pub use properties::{Label, LiteralProperty};
