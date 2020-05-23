/*!
Te `Error` and `Result` types for the entire toolkit.

# Example

TBD

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
    }
}
