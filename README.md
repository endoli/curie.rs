# curie

[![CI](https://github.com/endoli/curie.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/endoli/curie.rs/actions/workflows/ci.yml)
[![](https://img.shields.io/crates/v/curie.svg)](https://crates.io/crates/curie)
[![docs.rs](https://img.shields.io/docsrs/curie)](https://docs.rs/curie)

CURIEs, [defined by the W3C](https://www.w3.org/TR/curie/), are a compact
way of representing a URI.  A CURIE consists of an optional prefix and a
reference, separated by a colon.

They are commonly used in JSON-LD, RDF, SPARQL, XML namespaces and other
applications.

Dual licensed under the MIT and Apache 2 licenses.

## Documentation

The API is fully documented with examples:
[https://endoli.github.io/curie.rs/](https://endoli.github.io/curie.rs/)

## Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/curie).
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
curie = "0.1.1"
```

## Contribution

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.
