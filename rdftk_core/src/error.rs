/*!
The shared `Error`, `ErrorKind`, and `Result` common to the entire toolkit.
*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
        #[doc = "The String value provided is not a valid value for it's type."]
        InvalidFromStr(value: String, type_name: String) {
            description("The String value provided is not a value for it's type.")
            display("The String value `{}` is not a valid value for it's type: '{}'.", value, type_name)
        }
        #[doc = "The String value provided is not a valid Blank Node name."]
        InvalidBlankNodeName(s: String) {
            description("The String value provided is not a valid Blank Node name.")
            display("The String value `{}` is not a valid Blank Node name.", s)
        }
        #[doc = "A QName may not have an empty name part."]
        EmptyQName {
            description("A QName may not have an empty name part.")
            display("A QName may not have an empty name part.")
        }
        #[doc = "The String value provided is not a valid QName."]
        InvalidQName(s: String) {
            description("The String value provided is not a valid QName.")
            display("The String value `{}` is not a valid QName.", s)
        }
        #[doc = "Values from these different providers cannot be combined."]
        ProviderMismatch(lhs: String, rhs: String) {
            description("Values from these different providers cannot be combined.")
            display("Values from these different providers cannot be combined ({:?}, {:?}).", lhs, rhs)
        }
        #[doc = "The match combination is not valid."]
        InvalidMatch {
            description("The match combination is not valid.")
            display("The match combination is not valid.")
        }
        #[doc = "An Absolute IRI was expected at."]
        AbsoluteIriExpected(s: String) {
            description("An Absolute IRI was expected at.")
            display("An Absolute IRI was expected at, not '{}'.", s)
        }
        #[doc = "A failure occurred reading or writing a model.graph."]
        ReadWrite(repr: String) {
            description("A failure occurred reading or writing a model.graph.")
            display("A failure occurred reading or writing a model.graph, for representation: '{}'.", repr)
        }
        #[doc = "Some model element was in an invalid state for the requested operation."]
        InvalidState {
            description("Some model element was in an invalid state for the requested operation.")
            display("Some model element was in an invalid state for the requested operation.")
        }
        #[doc = "Statements as objects, from RDF*, are not supported by this representation."]
        RdfStarNotSupported(representation: String) {
            description("Statements as objects, from RDF*, are not supported by this representation.")
            display("Statements as objects, from RDF*, are not supported by the {:?} representation.", representation)
        }
        #[doc = "Cited model.formulae, from N3, are not supported by this representation."]
        FormulaeNotSupported(representation: String) {
            description("Cited model.formulae, from N3, are not supported by this representation.")
            display("Cited model.formulae, from N3, are not supported by the {:?} representation.", representation)
        }
    }

    foreign_links {
        Iri(::rdftk_iri::error::Error) #[doc = "A wrapped error parsing IRI strings."];
    }
}

#[allow(unused_macros)]
macro_rules! invalid_str_err {
    ($s:expr) => {
        Err($crate::error::ErrorKind::InvalidFromStr(
            $s.to_string(),
            ::std::any::type_name::<Self>().to_string(),
        )
        .into())
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Display an error trace to stdout.
///
pub fn print_trace(e: &dyn std::error::Error) {
    println!("{}", error_trace(e));
}

///
/// Display an error trace to stderr.
///
pub fn eprint_trace(e: &dyn std::error::Error) {
    eprintln!("{}", error_trace(e));
}

///
/// Convert an error into a trace string.
///
pub fn error_trace(e: &dyn std::error::Error) -> String {
    trace_one(e, 1)
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn trace_one(e: &dyn std::error::Error, count: i32) -> String {
    use std::fmt::Write;

    let mut trace = String::new();

    writeln!(&mut trace, "{}. {}", count, e.to_string()).expect("Failed to write message.");

    #[cfg(feature = "error-backtrace")]
    if let Some(backtrace) = e.backtrace() {
        writeln!(&mut trace, "{}", backtrace).expect("Failed to write backtrace.");
    }

    if let Some(source) = e.source() {
        write!(&mut trace, "{}", trace_one(source, count + 1)).expect("Failed to write source.");
    }

    writeln!(&mut trace).expect("Failed to write line break");
    trace
}
