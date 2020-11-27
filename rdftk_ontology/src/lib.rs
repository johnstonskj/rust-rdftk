/*!
Placeholder for query API and SPARQL support.

# Example

TBD

*/

use rdftk_core::{Literal, Statement};
use rdftk_iri::IRIRef;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum LabelProperty {
    Label(Literal),
    Comment(Literal),
    SeeAlso(IRIRef),
    IsDefinedBy(IRIRef),
}

pub trait Resource {
    fn uri(&self) -> &IRIRef;
}

pub trait Labeled: Resource {
    fn add_label(&mut self, value: Literal) {
        self.add_label_property(LabelProperty::Label(value))
    }
    fn add_comment(&mut self, value: Literal) {
        self.add_label_property(LabelProperty::Comment(value))
    }
    fn add_see_also(&mut self, value: IRIRef) {
        self.add_label_property(LabelProperty::SeeAlso(value))
    }
    fn add_is_defined_by(&mut self, value: IRIRef) {
        self.add_label_property(LabelProperty::IsDefinedBy(value))
    }
    fn add_label_property(&mut self, property: LabelProperty);
    fn remove_label_property(&mut self, property: &LabelProperty);
    fn label_properties(&self) -> Vec<&LabelProperty>;
}

pub trait Individual: Labeled {
    fn add_instance_of(&mut self, parent: IRIRef);
    fn remove_instance_of(&mut self, parent: &IRIRef);
    fn instance_of(&self) -> Vec<&IRIRef>;
    fn is_instance_of(&self, parent: &IRIRef) -> bool {
        self.instance_of().iter().any(|p| p == &parent)
    }
}

pub trait Subclassed: Individual {
    fn add_parent(&mut self, parent: IRIRef);
    fn remove_parent(&mut self, parent: &IRIRef);
    fn parents(&self) -> Vec<&IRIRef>;
    fn is_child_of(&self, parent: &IRIRef) -> bool {
        self.parents().iter().any(|p| p == &parent)
    }
}

pub trait ToStatements {
    fn to_statements(&self) -> Vec<Statement>;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod rdfs;

pub mod owl;
