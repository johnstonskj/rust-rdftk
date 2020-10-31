# RDFtk: IRI

Another implementation of the `IRI` and URI specifications.

[![crates.io](https://img.shields.io/crates/v/rdftk_iri.svg)](https://crates.io/crates/rdftk_iri)
[![docs.rs](https://docs.rs/rdftk_iri/badge.svg)](https://docs.rs/rdftk_iri)

As with the rest of the RDFtk project the aim of this crate is usability over optimization and so it may perform
more clones than necessary and parse more slowly than could be the case. For the most part clients should use the
`IRIRef` type that is an `Arc` reference and so can be reused without cloning the whole `IRI` value.

# Example

The most common use is the parsing of an `IRI` value from a string.

```rust
use rdftk_iri::IRI;
use std::convert::from_str;

let result = IRI::from_str(
    "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top",
);
```

The `builder` module allows for more programmatic construction of `IRI`s.

```rust
use rdftk_iri::{IRI, Scheme};
use rdftk_iri::builder::IriBuilder;

let mut builder = IriBuilder::default();
let result: IriResult<IRI> = builder
    .scheme(&Scheme::https())
    .user_name("john.doe")
    .host("www.example.com")?
    .port(123.into())
    .path_str("/forum/questions/")?
    .query_str("tag=networking&order=newest")?
    .fragment_str("top")?
    .try_into();
```

Note also the use of `Scheme::https()`, both the `Scheme` and `Port` types include associated functions
to construct well-known values.

# Features

The following features are present in this crate.

* `builder` [default] -- include the `builder` module, which in turn includes the `IriBuilder` type.
* `path_iri` [default] -- provides an implementation of `TryFrom<&PathBuf>` and `TryFrom<PathBuf>`
  for `IRI`.
* `uuid_iri` [default] -- provides an implementation of `TryFrom<&Uuid>` and `TryFrom<Uuid>`
  for `IRI`.

## Changes

**Version 0.1.4**

* A lot more testing, and local coverage reporting.
* Fixed a bug where separator missing in `UserInfo::to_string`.
* Fixed a parsing bug `IpvFuture::from_str`.
* Added `host`, `path_root`, `path` methods to `IriBuilder`.
* Changes `with_new_query` and `with_new_fragment` on `IRI` to not take `Option`.
* Added `blob` known value to `Scheme`.

**Version 0.1.3**

* Mostly testing
  1. Moved any tests out of the main code if they only use the public API.
  1. Added a set of files for gathering whole `IRI` examples.
  1. Added [proptest](https://docs.rs/proptest/0.10.1/proptest/index.html) for `Scheme`, will add for more.
* Fixed bug in `IRI::is_absolute`, to ignore authority and take the fragment into account.
* Added `IRI::is_relative_reference`.

**Version 0.1.2**

* Mostly documentation additions.
* Adding test cases where possible.
* Added helper functions and API shortcuts where they make sense.
* Added `path_iri` and `uuid_iri` features.

**Version 0.1.1**

* Added `IRIRef` type.

**Version 0.1.0**

* First release.

## TODO

1. [Complete IRI normalization](https://github.com/johnstonskj/rust-rdftk/issues/4)
1. [Complete IRI resolver](https://github.com/johnstonskj/rust-rdftk/issues/5)
1. [Complete IRI relativizer](https://github.com/johnstonskj/rust-rdftk/issues/6)

[![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
