# RDFtk: IRI

![iri](https://img.shields.io/badge/RDFtk-iri-BD1B89?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAQCAYAAAAmlE46AAAABGdBTUEAALGPC/xhBQAABBlpQ0NQa0NHQ29sb3JTcGFjZUdlbmVyaWNSR0IAADiNjVVdaBxVFD67c2cjJM5TbDSFdKg/DSUNk1Y0obS6f93dNm6WSTbaIuhk9u7OmMnOODO7/aFPRVB8MeqbFMS/t4AgKPUP2z60L5UKJdrUICg+tPiDUOiLpuuZOzOZabqx3mXufPOd75577rln7wXouapYlpEUARaari0XMuJzh4+IPSuQhIegFwahV1EdK12pTAI2Twt3tVvfQ8J7X9nV3f6frbdGHRUgcR9is+aoC4iPAfCnVct2AXr6kR8/6loe9mLotzFAxC96uOFj18NzPn6NaWbkLOLTiAVVU2qIlxCPzMX4Rgz7MbDWX6BNauuq6OWiYpt13aCxcO9h/p9twWiF823Dp8+Znz6E72Fc+ys1JefhUcRLqpKfRvwI4mttfbYc4NuWm5ERPwaQ3N6ar6YR70RcrNsHqr6fpK21iiF+54Q28yziLYjPN+fKU8HYq6qTxZzBdsS3NVry8jsEwIm6W5rxx3L7bVOe8ufl6jWay3t5RPz6vHlI9n1ynznt6Xzo84SWLQf8pZeUgxXEg4h/oUZB9ufi/rHcShADGWoa5Ul/LpKjDlsv411tpujPSwwXN9QfSxbr+oFSoP9Es4tygK9ZBqtRjI1P2i256uv5UcXOF3yffIU2q4F/vg2zCQUomDCHvQpNWAMRZChABt8W2Gipgw4GMhStFBmKX6FmFxvnwDzyOrSZzcG+wpT+yMhfg/m4zrQqZIc+ghayGvyOrBbTZfGrhVxjEz9+LDcCPyYZIBLZg89eMkn2kXEyASJ5ijxN9pMcshNk7/rYSmxFXjw31v28jDNSpptF3Tm0u6Bg/zMqTFxT16wsDraGI8sp+wVdvfzGX7Fc6Sw3UbbiGZ26V875X/nr/DL2K/xqpOB/5Ffxt3LHWsy7skzD7GxYc3dVGm0G4xbw0ZnFicUd83Hx5FcPRn6WyZnnr/RdPFlvLg5GrJcF+mr5VhlOjUSs9IP0h7QsvSd9KP3Gvc19yn3Nfc59wV0CkTvLneO+4S5wH3NfxvZq8xpa33sWeRi3Z+mWa6xKISNsFR4WcsI24VFhMvInDAhjQlHYgZat6/sWny+ePR0OYx/mp/tcvi5WAYn7sQL0Tf5VVVTpcJQpHVZvTTi+QROMJENkjJQ2VPe4V/OhIpVP5VJpEFM7UxOpsdRBD4ezpnagbQL7/B3VqW6yUurSY959AlnTOm7rDc0Vd0vSk2IarzYqlprq6IioGIbITI5oU4fabVobBe/e9I/0mzK7DxNbLkec+wzAvj/x7Psu4o60AJYcgIHHI24Yz8oH3gU484TastvBHZFIfAvg1Pfs9r/6Mnh+/dTp3MRzrOctgLU3O52/3+901j5A/6sAZ41/AaCffFUDXAvvAAAAIGNIUk0AAHomAACAhAAA+gAAAIDoAAB1MAAA6mAAADqYAAAXcJy6UTwAAAAJcEhZcwAADsQAAA7EAZUrDhsAAAFZaVRYdFhNTDpjb20uYWRvYmUueG1wAAAAAAA8eDp4bXBtZXRhIHhtbG5zOng9ImFkb2JlOm5zOm1ldGEvIiB4OnhtcHRrPSJYTVAgQ29yZSA1LjQuMCI+CiAgIDxyZGY6UkRGIHhtbG5zOnJkZj0iaHR0cDovL3d3dy53My5vcmcvMTk5OS8wMi8yMi1yZGYtc3ludGF4LW5zIyI+CiAgICAgIDxyZGY6RGVzY3JpcHRpb24gcmRmOmFib3V0PSIiCiAgICAgICAgICAgIHhtbG5zOnRpZmY9Imh0dHA6Ly9ucy5hZG9iZS5jb20vdGlmZi8xLjAvIj4KICAgICAgICAgPHRpZmY6T3JpZW50YXRpb24+MTwvdGlmZjpPcmllbnRhdGlvbj4KICAgICAgPC9yZGY6RGVzY3JpcHRpb24+CiAgIDwvcmRmOlJERj4KPC94OnhtcG1ldGE+CkzCJ1kAAAMUSURBVCgVPZJdaBRXFMfPuR8zO9k1GjfGqmjMKmqJojUtFPOgpYXYgBqpSUBB0ZqAivgiGh+C22LRvIs0YrG00IctVhAbrKCiLaI1fhLUVmMajMY0uslms7PzeU/vpMbhzr1z7/mdc/5zzwF4+xABZqiRp6+AmDx7t6aBtXaDjPZEhN0vO8snbOkrayIYJzYTxhulnX9s2nni6hetz+1LcybPC4XHs3/4c8fpc/f3V72DI+P5B+01A2N/bXs93tvsif4K1LFiamGRobxOyhtiwtxs8vj5fWu61mEm02hk54imfHHwy7w7uBqsQbTHxwBUPNDCQIEtTBOAGzpycV5Qv/zQ/FVzd72YyHjswod3RPngB69evQDlQVGwci09kJEbA+kFVOQlVimfa9U2t64+k4nUsfHTLSva1navLDHW188yP+mpSC6xwHgtQxoNiLyAxd4YiZIkT4SVOyadbu86W4PZgykKZTJTXlnXhi1H+n568tW67PNbR3P4tNoLR4A5yXtU9XBLuhoe3m0/89Hwtb79wYDThP/uNtRU5qFtpSBMzP45WVV3ELe29/3S07Et5/bg9pofvx/e82jRvb6uDudxvkE888EBRTi0t4zAtX0iV5bF9P9bC8Gbmjo7o/9NM5zshssbjmfcv0ca8JEHBe0CiL4oNaVAfQGkLwJZnEZ9CsF+qip4bmN+8XDdOfgWFv9uN/yTzXnM5AyBcXJJ6oRRl7BQvxwgRCAlQFi+axNIG2wFAYwqG1ByBFezk1WXqJjJbA7k+4BcRQUHckDq2LoOqAcKPYNPUQUATFQaCCAbMubGUr3T4yVSqIImUCOmpt6CERx9MtSdDD5ziCUgJhJr33PYjGPfLcvNrG1TUxaNTIv5WoTDAzD+TwcGKt01pEI+hSzJl8Tzsn5muvZo0/sCcVVRx+wYu3n8VO5C5hCygd0GPbOcMfALMA7mEIKxIB7SvNITSzfXfpNq+XgIuvYCUjrN4GWa40nwI2Ujvx6pVL1PLiYqra+v/7YRRKH/8LTqBZ8vO/Bpb2TvhFZZ1viZ+g+UE055oMSTLwAAAABJRU5ErkJggg==)
This crate provides an implementation of the `IRI` and `URI` specifications.

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
* `genid` [default] -- includes a constructor to create `"genid"` well-known IRI values.
* `path_iri` [default] -- provides an implementation of `TryFrom<&PathBuf>` and `TryFrom<PathBuf>`
  for `IRI`.
* `uuid_iri` [default] -- provides an implementation of `TryFrom<&Uuid>` and `TryFrom<Uuid>`
  for `IRI`.

## Changes

**Version 0.1.9**

* Added a feature to enable genid creation.
* Made IRI PartialOrd + Ord, it can now be sorted.
* Added PercentEncoding trait for percent encoding components.

**Version 0.1.8**

* Minor fix to parser to fix some precedence rules.
* Some documentation fixes.

**Version 0.1.7**

* Added support for well-known IRIs to the Path and IRI types.

**Version 0.1.6**

* Applied a lot more warnings in lib.rs
* Applied more Clippy suggestions.

**Version 0.1.5**

* Applied all Clippy suggestions.

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
