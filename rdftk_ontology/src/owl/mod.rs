/*!
One-line description.

More detailed description, with

# Example

*/

use crate::LabelProperty;
use rdftk_core::Literal;
use rdftk_iri::IRIRef;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// Thing

// Nothing

#[derive(Clone, Debug, PartialEq)]
pub enum HeaderProperty {
    VersionInfo(Literal),
    PriorVersion(IRIRef),
    BackwardCompatibleWith(IRIRef),
    IncompatibleWith(IRIRef),
    Imports(IRIRef),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ontology {
    uri: IRIRef,
    label_properties: Vec<LabelProperty>,
    header_properties: Vec<HeaderProperty>,
    classes: HashMap<IRIRef, Class>,
    properties: HashMap<IRIRef, Property>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    description: ClassDescription,
    axioms: Vec<ClassAxiom>,
    deprecated: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    deprecated: bool,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum ClassDescription {
    Identifier(IRIRef),
    Enumeration(Vec<ClassDescription>),
    PropertyRestriction,
    Intersection(Vec<ClassDescription>),
    Union(Vec<ClassDescription>),
    Compliment(Box<ClassDescription>),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum ClassAxiom {
    SubClassOf(ClassDescription),
    EquivalentClass(ClassDescription),
    DisjointWith(ClassDescription),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
