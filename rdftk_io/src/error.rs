/*!
The shared `Error`, `ErrorKind`, and `Result` types for I/O operations.

# Example

TBD

*/

#![allow(missing_docs)]

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
        #[doc = "Statements as objects from RDF* are not supported by this representation."]
        RdfStarNotSupported(representation: String) {
            description("Statements as objects from RDF* are not supported by this representation.")
            display("Statements as objects from RDF* are not supported by the {:?} representation.", representation)
        }
    }

    foreign_links {
        Json(::serde_json::error::Error);
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        XmlWrite(::xml::writer::Error) #[cfg(feature = "xml")];
    }
}
