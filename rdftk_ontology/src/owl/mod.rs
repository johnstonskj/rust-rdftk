/*!
One-line description.

More detailed description, with

# Example

*/

use crate::LabelProperty;
use rdftk_core::model::literal::LiteralRef;
use rdftk_iri::IRIRef;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// Thing

// Nothing

#[derive(Clone, Debug)]
pub enum HeaderProperty {
    VersionInfo(LiteralRef),
    PriorVersion(IRIRef),
    BackwardCompatibleWith(IRIRef),
    IncompatibleWith(IRIRef),
    Imports(IRIRef),
}

#[derive(Clone, Debug)]
pub struct Ontology {
    #[allow(unused)]
    uri: IRIRef,
    #[allow(unused)]
    label_properties: Vec<LabelProperty>,
    #[allow(unused)]
    header_properties: Vec<HeaderProperty>,
    #[allow(unused)]
    classes: HashMap<IRIRef, Class>,
    #[allow(unused)]
    properties: HashMap<IRIRef, Property>,
}

#[derive(Clone, Debug)]
pub struct Class {
    #[allow(unused)]
    description: ClassDescription,
    #[allow(unused)]
    axioms: Vec<ClassAxiom>,
    #[allow(unused)]
    deprecated: bool,
}

#[derive(Clone, Debug)]
pub struct Property {
    #[allow(unused)]
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
