use rdftk_iri::IRIRef;
use rdftk_names::xsd;

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
    /// Denotes a literal where the type is indicated by the provided `IRI`.
    Other(IRIRef),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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
            DataType::Other(iri) => iri,
        }
    }
}
