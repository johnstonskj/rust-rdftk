/*!
The shared `Error`, `ErrorKind`, and `Result` types for I/O operations.

# Example

TBD

*/

#![allow(missing_docs)]

use rdftk_core::error::{Error as CoreError, ErrorKind as CoreErrorKind};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
        #[doc("An error occurred serializing a graph")]
        Serialization(representation: String) {
            description("An error occurred serializing a graph")
            display("An error occurred serializing a graph into {}", representation)
        }
        #[doc("An error occurred de-serializing a graph")]
        Deserialization(representation: String, location: String, context: Option<String>) {
            description("An error occurred de-serializing a graph")
            display("An error occurred de-serializing a graph from {} at location '{}' (context '{:?}')", representation, location, context)
        }
        #[doc = "Statements as objects from RDF* are not supported by this representation."]
        RdfStarNotSupported(representation: String) {
            description("Statements as objects from RDF* are not supported by this representation.")
            display("Statements as objects from RDF* are not supported by the {:?} representation.", representation)
        }
    }

    links {
        Core(CoreError, CoreErrorKind);
    }
    foreign_links {
        Json(::serde_json::error::Error);
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        XmlWrite(::xml::writer::Error) #[cfg(feature = "xml")];
    }
}

impl From<Error> for CoreError {
    fn from(e: Error) -> Self {
        CoreError::with_chain(e, CoreErrorKind::Io)
    }
}
