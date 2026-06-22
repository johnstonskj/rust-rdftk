//!
//! ![iri](https://img.shields.io/badge/RDFtk-iri-BD1B89?logo=data:image/png;base64,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)
//! This crate provides an `IriExtra` trait, `Iri`, `Namespace`, `Name`, and `QName`
//! types and associated errors.
//!
//! The primary reference for this crate is the [SPARQL](https://www.w3.org/TR/rdf-sparql-query/)
//! specification. The [Turtle](https://www.w3.org/TR/turtle/) language as well as the OWL
//! [Functional Syntax](https://www.w3.org/TR/owl2-syntax/) and
//! [Manchester Syntax](http://www.w3.org/TR/owl2-manchester-syntax/) reference the SPARQL
//! `PNAME_NS`, `PNAME_LN`, and `PrefixedName` productions.
//!
//! ## Specification Summary
//!
//! From [SPARQL](https://www.w3.org/TR/rdf-sparql-query/) §[A.8 Grammar](https://www.w3.org/TR/2008/REC-rdf-sparql-query-20080115/#sparqlGrammar):
//!
//! ```text
//! [68]  	PrefixedName	  ::=  	PNAME_LN | PNAME_NS
//!
//! [71]  	PNAME_NS	  ::=  	PN_PREFIX? ':'
//! [72]  	PNAME_LN	  ::=  	PNAME_NS PN_LOCAL
//! [95]  	PN_CHARS_BASE ::=  	[A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6] | [#x00F8-#x02FF]
//!                             | [#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F]
//!                             | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD]
//!                             | [#x10000-#xEFFFF]
//! [96]  	PN_CHARS_U	  ::=  	PN_CHARS_BASE | '_'
//!
//! [98]  	PN_CHARS	  ::=  	PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]
//! [99]  	PN_PREFIX	  ::=  	PN_CHARS_BASE ((PN_CHARS|'.')* PN_CHARS)?
//! [100]  	PN_LOCAL	  ::=  	( PN_CHARS_U | [0-9] ) ((PN_CHARS|'.')* PN_CHARS)?
//!         /* Note that SPARQL local names allow leading digits while XML local names do not. */
//! ```
//!
//! Where the following table provides a mapping from the BNF above to types in this package.
//!
//! | Specification Name | Crate Type              | Comments                                                        |
//! | ------------------ | ----------------------- | --------------------------------------------------------------- |
//! | `IRIRef`           | [`iri::IriRef`]         | An enum with `Iri` and `PrefixedName` variants.                 |
//! | `IRI_REF`          | [`iri::Iri`]            | A wrapper around the `Url` type from the `url` crate.           |
//! |                    | [`iri::IriExtra`]       | A trait implemented by `Iri` to add to the `Url` type.          |
//! | `PrefixedName`     | [`pname::PrefixedName`] | A tuple of `Namespace` and `Name`.                              |
//! | `PNAME_LN`         | [`pname::Name`]         | The name of the locally-defined *thing*.                        |
//! | `PNAME_NS`         | [`pname::Namespace`]    | The prefix name used to reference the namespace, including ':'. |
//!
//! ## The `Iri` Type
//!
//! The [`Iri`] type is a wrapper around the `url::Url` type, and as can be constructed in the
//! same manner.
//!
//! ### Examples
//!
//! ```rust
//! use rdftk_iri::Iri;
//! use std::str::FromStr;
//!
//! let result = Iri::from_str(
//!     "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top",
//! ).unwrap();
//! ```
//!
//! ### The [`IriExtra`] trait provides a number of additional methods useful to the IRI as a namespace
//! and namespaced-name identifier.
//!
//! ```rust
//! use rdftk_iri::{Iri, IriExtra as _, Name};
//! use std::str::FromStr;
//!
//! let namespace = Iri::from_str(
//!     "https://example.org/ns/things#",
//! ).unwrap();
//! assert!(namespace.looks_like_namespace());
//!
//! let name = namespace.make_name(Name::from_str("ThisThing").unwrap()).unwrap();
//! assert_eq!(
//!     name.to_string(),
//!     "https://example.org/ns/things#ThisThing".to_string(),
//! );
//!
//! assert_eq!(
//!     name.namespace(),
//!     Some(namespace),
//! );
//!
//! assert_eq!(
//!     name.name(),
//!     Some(Name::from_str("ThisThing").unwrap()),
//! );
//! ```
//!
//! ### Curie
//!
//! Specification: <https://www.w3.org/TR/curie/>
//!
//! ```text
//! safe_curie  :=   '[' curie ']'
//!
//! curie       :=   [ [ prefix ] ':' ] reference
//!
//! prefix      :=   NCName
//!
//! reference   :=   irelative-ref (as defined in IRI)
//! ```
//!
//! Note that while the empty string matches the production for curie above, an empty string is NOT a
//! valid CURIE. The CURIE prefix '_' is reserved for use by languages that support RDF. For this
//! reason, the prefix '_' SHOULD be avoided by authors.
//!
//! ## The `Namespace` Type
//!
//! A [`Namespace`], `PNAME_NS` from the [SPARQL](https://www.w3.org/TR/rdf-sparql-query/)
//! specification, is a prefix string used to reference a namespace IRI in languages such as
//! OWL, Turtle, SPARQL and so forth.
//!
//! ```rust
//! use rdftk_iri::{LocalName, Name, Namespace};
//! use std::str::FromStr;
//!
//! let ns = Namespace::from_str(":").unwrap();
//! assert!(ns.is_default());
//!
//! let ns = Namespace::from_str("rdf:").unwrap();
//! assert!(!ns.is_default());
//!
//! let name: LocalName = ns.qualify(&Name::from_str("type").unwrap());
//! assert_eq!("rdf:type".to_string(), name.to_string());
//! ```
//!
//! ## `Name` Type
//!
//! A [`Name`], `PN_LOCAL` from the [SPARQL](https://www.w3.org/TR/rdf-sparql-query/)
//! specification, represents that portion of a [`LocalName`] that refers to the
//! locally defined or referenced *thing*.
//!
//! ```rust
//! use rdftk_iri::{LocalName, Name, Namespace};
//! use std::str::FromStr;
//!
//! assert!(Name::from_str("subClassOf").is_ok());
//!
//! assert!(Name::from_str("rdf:").is_err());
//!
//! let name = Name::from_str("type").unwrap();
//! let name: LocalName = name.qualify(&Namespace::from_str("rdf:").unwrap());
//! assert_eq!("rdf:type".to_string(), name.to_string());
//! ```
//!
//! ## The `LocalName` Type
//!
//! A [`LocalName`], `PNAME_LN` from the [SPARQL](https://www.w3.org/TR/rdf-sparql-query/)
//! specification, represents the tuple of [`Namespace`] and [`Name`], such that every
//! name is qualified with the namespace it is defined within.
//!
//! ### Example
//!
//! ```rust
//! use rdftk_iri::{LocalName, Name, Namespace};
//! use std::str::FromStr;
//!
//! let prefixed: LocalName = "prefix:name".parse().expect("parse error");
//! let un_prefixed: LocalName = ":name".parse().expect("parse error");
//!
//! let prefixed: LocalName = LocalName::new(
//!     Namespace::new_unchecked("prefix"),
//!     Name::new_unchecked("name"),
//! );
//! let un_prefixed: LocalName = LocalName::new_in_default(Name::new_unchecked("name"));
//!
//! assert!(LocalName::from_str("").is_err());
//! assert!(LocalName::from_str("hello world").is_err());
//! ```
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//
// Support for no-std needs to be first here.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;
pub use error::Error;

pub mod iri;
pub use iri::{Iri, IriExtra, IriRef};

pub mod map;
pub use map::IriPrefixMap;

pub mod pname;
pub use pname::{LocalName, Name, Namespace, PrefixedName};

pub mod vocab;
pub use vocab::Vocabulary;
