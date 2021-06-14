use rdftk_iri::IRIRef;
use rdftk_names::{rdf, xsd};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
///
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    /// Denotes a literal of type `xsd::string`.
    String,
    /// Denotes a literal of type `xsd::qname`.
    QName,
    /// Denotes a literal of type `xsd::anyURI`.
    #[allow(clippy::upper_case_acronyms)]
    IRI,
    /// Denotes a literal of type `xsd::boolean`.
    Boolean,
    /// Denotes a literal of type `xsd::float`.
    Float,
    /// Denotes a literal of type `xsd::double`.
    Double,
    /// Denotes a literal of type `xsd::long`.
    Long,
    /// Denotes a literal of type `xsd::int`.
    Int,
    /// Denotes a literal of type `xsd::short`.
    Short,
    /// Denotes a literal of type `xsd::byte`.
    Byte,
    /// Denotes a literal of type `xsd::unsignedLong`.
    UnsignedLong,
    /// Denotes a literal of type `xsd::unsignedInt`.
    UnsignedInt,
    /// Denotes a literal of type `xsd::unsignedShort`.
    UnsignedShort,
    /// Denotes a literal of type `xsd::unsignedByte`.
    UnsignedByte,
    /// Denotes a literal of type `xsd::duration`.
    Duration,
    /// Denotes an escaped string containing XML content.
    XmlLiteral,
    /// Denotes a literal where the type is indicated by the provided `IRI`.
    Other(IRIRef),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<IRIRef> for DataType {
    fn from(iri: IRIRef) -> Self {
        if &iri == xsd::string() {
            DataType::String
        } else if &iri == xsd::q_name() {
            DataType::QName
        } else if &iri == xsd::any_uri() {
            DataType::IRI
        } else if &iri == xsd::boolean() {
            DataType::Boolean
        } else if &iri == xsd::float() {
            DataType::Float
        } else if &iri == xsd::double() {
            DataType::Double
        } else if &iri == xsd::long() {
            DataType::Long
        } else if &iri == xsd::int() {
            DataType::Int
        } else if &iri == xsd::short() {
            DataType::Short
        } else if &iri == xsd::byte() {
            DataType::Byte
        } else if &iri == xsd::unsigned_long() {
            DataType::UnsignedLong
        } else if &iri == xsd::unsigned_int() {
            DataType::UnsignedInt
        } else if &iri == xsd::unsigned_short() {
            DataType::UnsignedShort
        } else if &iri == xsd::unsigned_byte() {
            DataType::UnsignedByte
        } else if &iri == xsd::duration() {
            DataType::Duration
        } else if &iri == rdf::xml_literal() {
            DataType::XmlLiteral
        } else {
            DataType::Other(iri)
        }
    }
}

impl DataType {
    ///
    /// Return the IRI representing this data type. Primarily these are the XML Schema data types
    /// used for literal values.
    ///
    pub fn as_iri(&self) -> &IRIRef {
        match &self {
            DataType::String => xsd::string(),
            DataType::QName => xsd::q_name(),
            DataType::IRI => xsd::any_uri(),
            DataType::Boolean => xsd::boolean(),
            DataType::Float => xsd::float(),
            DataType::Double => xsd::double(),
            DataType::Long => xsd::long(),
            DataType::Int => xsd::int(),
            DataType::Short => xsd::short(),
            DataType::Byte => xsd::byte(),
            DataType::UnsignedLong => xsd::unsigned_long(),
            DataType::UnsignedInt => xsd::unsigned_int(),
            DataType::UnsignedShort => xsd::unsigned_short(),
            DataType::UnsignedByte => xsd::unsigned_byte(),
            DataType::Duration => xsd::duration(),
            DataType::XmlLiteral => rdf::xml_literal(),
            DataType::Other(iri) => iri,
        }
    }
}
