# curie

[![Build Status](https://travis-ci.org/endoli/curie.rs.svg?branch=master)](https://travis-ci.org/endoli/curie.rs)
[![](http://meritbadge.herokuapp.com/curie)](https://crates.io/crates/curie)

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
curie = "0.0.8"
```

## Status of Implementation

Things are under active development. This project is not quite
usable yet as some of the basic functionality is being written.

## Contribution

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.
