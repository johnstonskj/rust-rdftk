/*!
Provides the `IRI` specific `Error` and `Result` types.

# Example

TBD

*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum Component {
    Scheme,
    Authority,
    Path,
    Query,
    Fragment,
}

error_chain! {
    errors {
        #[doc = "An IRI cannot be constructed from the empty string."]
        IsEmpty {
            description("An IRI cannot be constructed from the empty string.")
            display("An IRI cannot be constructed from the empty string.")
        }
        #[doc = "An error occurred normalizing an IRI component."]
        Normalization(c: Component) {
            description("An error occurred normalizing an IRI component.")
            display("An error occurred normalizing the {:?} IRI component.", c)
        }
        #[doc = "An invalid character was found."]
        InvalidChar(c: Component) {
            description("An invalid character was found.")
            display("An invalid character was found in the {:?} IRI component.", c)
        }
        #[doc = "Provided String value is not a valid IRI."]
        Syntax(s: String) {
            description("Provided String value is not a valid IRI.")
            display("Provided String value `{}` was is a valid IRI.", s)
        }
    }
}
