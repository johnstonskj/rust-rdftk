/*!
The shared `Error`, `ErrorKind`, and `Result` common to the entire toolkit.
*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
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
        #[doc = "The match combination is not valid."]
        InvalidMatch {
            description("The match combination is not valid.")
            display("The match combination is not valid.")
        }
        #[doc = "A failure occurred reading or writing a graph."]
        Io {
            description("A failure occurred reading or writing a graph.")
            display("A failure occurred reading or writing a graph.")
        }
    }
}
