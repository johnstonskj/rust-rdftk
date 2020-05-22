/*!
One-line description.

More detailed description, with

# Example

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
        #[doc = "A URI cannot be constructed from the empty string."]
        IsEmpty {
            description("A URI cannot be constructed from the empty string.")
            display("A URI cannot be constructed from the empty string.")
        }
        #[doc = "An error occurred normalizing a URI component."]
        Normalization(c: Component) {
            description("An error occurred normalizing a URI component.")
            display("An error occurred normalizing the {:?} URI component.", c)
        }
        #[doc = "An invalid character was found."]
        InvalidChar(c: Component) {
            description("An invalid character was found.")
            display("An invalid character was found in the {:?} URI component.", c)
        }
        #[doc = "Provided String value is not a valid URI."]
        Syntax(s: String) {
            description("Provided String value is not a valid URI.")
            display("Provided String value `{}` was is a valid URI.", s)
        }
    }
}
