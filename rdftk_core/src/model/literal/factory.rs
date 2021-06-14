/*!
Graphs may have mechanisms to cache commonly used values, or those with significant storage
overhead. In such cases they provide a value factory that should be used to construct new values
for use in the associated graph. It is possible that all graphs provided by some graph store share
a common value factory by store rather than by graph.
*/

use crate::error::Result;
use crate::model::literal::{DataType, LanguageTag, LiteralRef};
use crate::model::Provided;
use rdftk_iri::IRIRef;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A value factory can be used to provide previously cached values rather than creating duplicates
/// within a graph. Such a factory may only be retrieved using the `Graph::literal_factory` method.
///
pub trait LiteralFactory: Debug + Provided {
    /// Returns a cached *untyped* literal value with the provided string.
    fn literal(&self, v: &str) -> LiteralRef;

    /// Returns a cached literal value with the provided string and language.
    fn with_language(&self, v: &str, lang: LanguageTag) -> LiteralRef;

    /// Returns a cached literal value with the provided string and language.
    fn with_language_str(&self, v: &str, lang: &str) -> Result<LiteralRef> {
        Ok(self.with_language(v, LanguageTag::from_str(lang)?))
    }

    /// Returns a cached literal value with the provided string and data type.
    fn with_data_type(&self, v: &str, data_type: DataType) -> LiteralRef;

    /// Returns a cached literal value with the provided string.
    fn string(&self, v: &str) -> LiteralRef {
        self.with_data_type(v, DataType::String)
    }

    /// Returns a cached literal value with the provided QName.
    fn qname(&self, v: &str) -> LiteralRef {
        self.with_data_type(v, DataType::QName)
    }

    /// Returns a cached literal value with the provided IRI.
    fn uri(&self, v: &IRIRef) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::IRI)
    }

    /// Returns a cached literal value with the provided boolean.
    fn boolean(&self, v: bool) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::Boolean)
    }

    /// Returns a cached literal value with the provided float.
    fn float(&self, v: f32) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::Float)
    }

    /// Returns a cached literal value with the provided double.
    fn double(&self, v: f64) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::Double)
    }

    /// Returns a cached literal value with the provided long.
    fn long(&self, v: i64) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::Long)
    }

    /// Returns a cached literal value with the provided int.
    fn int(&self, v: i32) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::Int)
    }

    /// Returns a cached literal value with the provided short.
    fn short(&self, v: i16) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::Short)
    }

    /// Returns a cached literal value with the provided byte.
    fn byte(&self, v: i8) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::Byte)
    }

    /// Returns a cached literal value with the provided unsigned long.
    fn unsigned_long(&self, v: u64) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::UnsignedLong)
    }

    /// Returns a cached literal value with the provided unsigned int.
    fn unsigned_int(&self, v: u32) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::UnsignedInt)
    }

    /// Returns a cached literal value with the provided unsigned short.
    fn unsigned_short(&self, v: u16) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::UnsignedShort)
    }

    /// Returns a cached literal value with the provided unsigned byte.
    fn unsigned_byte(&self, v: u8) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::UnsignedByte)
    }

    /// Returns a cached literal value with the provided duration.
    fn duration(&self, v: Duration) -> LiteralRef {
        self.chrono_duration(chrono::Duration::from_std(v).unwrap())
    }

    /// Returns a cached literal value with the provided duration.
    #[cfg(feature = "chrono_types")]
    fn chrono_duration(&self, v: chrono::Duration) -> LiteralRef {
        self.with_data_type(&v.to_string(), DataType::Duration)
    }
}

pub type LiteralFactoryRef = Arc<dyn LiteralFactory>;
