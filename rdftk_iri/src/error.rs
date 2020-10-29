/*!
Provides the `IRI` specific `Error` and `Result` types.

# Example

The following demonstrates the use of [`ErrorKind`](enum.ErrorKind.html) in constructing an error
to return to the caller.

```rust
use rdftk_iri::IRI;
use rdftk_iri::error::{ErrorKind, Result as IriResult};

fn some_operation() -> IriResult<IRI> {
    Err(ErrorKind::IsEmpty.into())
}
```

*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Determines the component of the URI signaling the error. This is used by lower-level errors
/// such as character parsing.
///
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
        #[doc = "An error occurred parsing the IRI scheme."]
        ParseSchemeError(s: String) {
            description("An error occurred parsing the IRI scheme.")
            display("An error occurred parsing the IRI scheme: {:?}.", s)
        }
        #[doc = "An error occurred parsing the IRI authority."]
        ParseAuthorityError(s: String) {
            description("An error occurred parsing the IRI authority.")
            display("An error occurred parsing the IRI authority: {:?}.", s)
        }
        #[doc = "An error occurred parsing the IRI IP address name."]
        ParseIpAddressError(s: String) {
            description("An error occurred parsing the IRI IP address name.")
            display("An error occurred parsing the IRI IP address name: {:?}.", s)
        }
        #[doc = "An error occurred parsing the IRI host name."]
        ParseHostError(s: String) {
            description("An error occurred parsing the IRI host name.")
            display("An error occurred parsing the IRI host name: {:?}.", s)
        }
        #[doc = "An error occurred parsing the IRI port number."]
        ParsePortError(s: String) {
            description("An error occurred parsing the IRI port number.")
            display("An error occurred parsing the IRI port number: {:?}.", s)
        }
        #[doc = "An error occurred parsing the IRI user info."]
        ParseUserInfoError(s: String) {
            description("An error occurred parsing the IRI user info.")
            display("An error occurred parsing the IRI user info: {:?}.", s)
        }
        #[doc = "An error occurred parsing the IRI fragment."]
        ParseFragmentError(s: String) {
            description("An error occurred parsing the IRI fragment.")
            display("An error occurred parsing the IRI fragment: {:?}.", s)
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
        #[doc = "The current IRI is not a valid base URI (RFC-3986§5.2.1)."]
        NotValidBase {
            description("The current IRI is not a valid base URI (RFC-3986§5.2.1).")
            display("The current IRI is not a valid base URI (RFC-3986§5.2.1).")
        }
        #[doc = "A PrefixedName may not have an empty name part."]
        EmptyPrefixedName {
            description("A PrefixedName may not have an empty name part.")
            display("A PrefixedName may not have an empty name part.")
        }
        #[doc = "The String value provided is not a valid PrefixedName."]
        InvalidPrefixedName(s: String) {
            description("The String value provided is not a valid PrefixedName.")
            display("The String value `{}` is not a valid PrefixedName.", s)
        }
    }
}
