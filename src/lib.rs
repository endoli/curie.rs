// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CURIEs: Compact URIs
//!
//! CURIEs, [defined by the W3C], are a compact way of representing a URI.
//! A CURIE consists of an optional prefix and a reference, separated by
//! a colon.
//!
//! They are commonly used in JSON-LD, RDF, SPARQL, XML namespaces and other
//! applications.
//!
//! Example CURIEs:
//!
//! * `"foaf:Person"` -- Results in a URI in the namespace represented by
//!   the `"foaf"` prefix.
//! * `":Person"` -- Results in a URI in the namespace represented by
//!   the `""` prefix.
//! * `"Person"` -- Results in a URI in the default namespace.
//!
//! The last example relies upon there being a default mapping providing
//! a default base URI, while the example before it relies upon there
//! being a prefix which is an empty string.
//!
//! See the [specification] for further details.
//!
//! ## Compact URIs in the Real World
//!
//! In SPARQL (from Wikipedia):
//!
//! ```sparql
//! PREFIX foaf: <http://xmlns.com/foaf/0.1/>
//! SELECT ?name
//!        ?email
//! WHERE
//!   {
//!     ?person  a          foaf:Person .
//!     ?person  foaf:name  ?name .
//!     ?person  foaf:mbox  ?email .
//!   }
//! ```
//!
//! In the Turtle serialization for RDF (from the specification):
//!
//! ```turtle
//! @base <http://example.org/> .
//! @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
//! @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
//! @prefix foaf: <http://xmlns.com/foaf/0.1/> .
//! @prefix rel: <http://www.perceive.net/schemas/relationship/> .
//!
//! <#green-goblin>
//!     rel:enemyOf <#spiderman> ;
//!     a foaf:Person ;    # in the context of the Marvel universe
//!     foaf:name "Green Goblin" .
//!
//! <#spiderman>
//!     rel:enemyOf <#green-goblin> ;
//!     a foaf:Person ;
//!     foaf:name "Spiderman", "Человек-паук"@ru .
//! ```
//!
//! ## Usage
//!
//! ```
//! use curie::PrefixMapping;
//!
//! // Initialize a prefix mapper.
//! let mut mapper = PrefixMapping::default();
//! mapper.add_prefix("foaf", "http://xmlns.com/foaf/0.1/").unwrap();
//!
//! // Set a default prefix
//! mapper.set_default("http://example.com/");
//!
//! // Expand a CURIE and get back the full URI.
//! assert_eq!(mapper.expand_curie_string("Entity"),
//!            Ok(String::from("http://example.com/Entity")));
//! assert_eq!(mapper.expand_curie_string("foaf:Agent"),
//!            Ok(String::from("http://xmlns.com/foaf/0.1/Agent")));
//! ```
//!
//! When parsing a file, it is likely that the distinction between
//! the prefix and the reference portions of the CURIE will be clear,
//! so to save time during expansion, the `Curie` struct can also be
//! used:
//!
//! ```
//! use curie::{Curie, PrefixMapping};
//!
//! // Initialize a prefix mapper.
//! let mut mapper = PrefixMapping::default();
//! mapper.add_prefix("foaf", "http://xmlns.com/foaf/0.1/").unwrap();
//!
//! let curie = Curie::new("foaf", "Agent");
//!
//! assert_eq!(mapper.expand_curie(&curie),
//!            Ok(String::from("http://xmlns.com/foaf/0.1/Agent")));
//! ```
//!
//! [defined by the W3C]: https://www.w3.org/TR/curie/
//! [specification]: https://www.w3.org/TR/curie/

#![warn(missing_docs)]
#![deny(trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]

use std::collections::HashMap;
use std::fmt;

/// Errors that might occur when adding a prefix to a [`PrefixMapping`].
///
/// [`PrefixMapping`]: struct.PrefixMapping.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InvalidPrefixError {
    /// This is a reserved prefix.
    ReservedPrefix,
}

/// Errors that might occur during CURIE expansion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExpansionError {
    /// The prefix on the CURIE has no valid mapping.
    Invalid,
    /// The CURIE uses a default prefix, but one has not
    /// been set.
    MissingDefault,
}

/// Maps prefixes to base URIs and allows for the expansion of
/// CURIEs (Compact URIs).
#[derive(Default)]
pub struct PrefixMapping {
    default: Option<String>,
    mapping: HashMap<String, String>,
}

impl PrefixMapping {
    /// Set a default prefix.
    ///
    /// This is used during CURIE expansion when there is no
    /// prefix, just a reference value.
    pub fn set_default(&mut self, default: &str) {
        self.default = Some(String::from(default));
    }

    /// Add a prefix to the mapping.
    ///
    /// This allows this prefix to be resolved when a CURIE is expanded.
    pub fn add_prefix(&mut self, prefix: &str, value: &str) -> Result<(), InvalidPrefixError> {
        if prefix == "_" {
            Err(InvalidPrefixError::ReservedPrefix)
        } else {
            self.mapping
                .insert(String::from(prefix), String::from(value));
            Ok(())
        }
    }

    /// Remove a prefix from the mapping.
    ///
    /// Future calls to [`expand_curie_string`] or [`expand_curie`] that use
    /// this `prefix` will result in a `ExpansionError::Invalid` error.
    ///
    /// [`expand_curie_string`]: struct.PrefixMapping.html#method.expand_curie_string
    /// [`expand_curie`]: struct.PrefixMapping.html#method.expand_curie
    pub fn remove_prefix(&mut self, prefix: &str) {
        self.mapping.remove(prefix);
    }

    /// Expand a CURIE, returning a complete IRI.
    pub fn expand_curie_string(&self, curie: &str) -> Result<String, ExpansionError> {
        if let Some(separator_idx) = curie.chars().position(|c| c == ':') {
            // If we have a separator, try to expand.
            let prefix = &curie[..separator_idx];
            let reference = &curie[separator_idx + 1..];
            self.expand_exploded_curie(prefix, reference)
        } else if let Some(ref default) = self.default {
            // No separator, so look for default.
            Ok(default.clone() + curie)
        } else {
            Err(ExpansionError::MissingDefault)
        }
    }

    /// Expand a parsed [`Curie`], returning a complete IRI.
    ///
    /// [`Curie`]: struct.Curie.html
    pub fn expand_curie(&self, curie: &Curie) -> Result<String, ExpansionError> {
        self.expand_exploded_curie(curie.prefix, curie.reference)
    }

    fn expand_exploded_curie(
        &self,
        prefix: &str,
        reference: &str,
    ) -> Result<String, ExpansionError> {
        if let Some(mapped_prefix) = self.mapping.get(prefix) {
            Ok((*mapped_prefix).clone() + reference)
        } else {
            Err(ExpansionError::Invalid)
        }
    }

    /// Return an iterator over the prefix mappings.
    ///
    /// This is useful when testing code that uses this crate.
    pub fn mappings(&self) -> ::std::collections::hash_map::Iter<String, String> {
        self.mapping.iter()
    }
}

/// A prefix and reference, already parsed into separate components.
#[derive(Debug)]
pub struct Curie<'c> {
    prefix: &'c str,
    reference: &'c str,
}

impl<'c> Curie<'c> {
    /// Construct a `Curie` from a prefix and reference.
    pub fn new(prefix: &'c str, reference: &'c str) -> Self {
        Curie { prefix, reference }
    }
}

impl<'c> fmt::Display for Curie<'c> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.prefix, self.reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOAF_VOCAB: &'static str = "http://xmlns.com/foaf/0.1/";

    #[test]
    fn add_remove_works() {
        let mut pm = PrefixMapping::default();

        // No keys should be found.
        assert_eq!(pm.mapping.get("foaf"), None);

        // Add and look up a key.
        assert_eq!(pm.add_prefix("foaf", FOAF_VOCAB), Ok(()));
        assert_eq!(pm.mapping.get("foaf"), Some(&String::from(FOAF_VOCAB)));

        // Unrelated keys still can not be found.
        assert_eq!(pm.mapping.get("rdfs"), None);

        // Can't add _ as that's reserved.
        assert_eq!(
            pm.add_prefix("_", ""),
            Err(InvalidPrefixError::ReservedPrefix)
        );

        // Keys can be removed.
        pm.remove_prefix("foaf");

        // The "foaf" key should not be found.
        assert_eq!(pm.mapping.get("foaf"), None);
    }

    #[test]
    fn display_curie() {
        let curie = Curie::new("foaf", "Agent");
        assert_eq!(String::from("foaf:Agent"), format!("{}", curie));
    }

    #[test]
    fn expand_curie_string() {
        let mut mapping = PrefixMapping::default();

        let curie = "foaf:Person";

        // A CURIE with an unmapped prefix isn't expanded.
        assert_eq!(
            mapping.expand_curie_string(curie),
            Err(ExpansionError::Invalid)
        );

        // A CURIE without a separator doesn't cause problems. It still
        // requires a default though.
        assert_eq!(
            mapping.expand_curie_string("Person"),
            Err(ExpansionError::MissingDefault)
        );

        mapping.set_default("http://example.com/");

        assert_eq!(
            mapping.expand_curie_string("Person"),
            Ok(String::from("http://example.com/Person"))
        );

        // Using a colon without a prefix results in using a prefix
        // for an empty string.
        assert_eq!(
            mapping.expand_curie_string(":Person"),
            Err(ExpansionError::Invalid)
        );
        mapping
            .add_prefix("", "http://example.com/ExampleDocument#")
            .unwrap();
        assert_eq!(
            mapping.expand_curie_string(":Person"),
            Ok(String::from("http://example.com/ExampleDocument#Person"))
        );

        // And having a default won't allow a prefixed CURIE to
        // be expanded with the default.
        assert_eq!(
            mapping.expand_curie_string(curie),
            Err(ExpansionError::Invalid)
        );

        mapping.add_prefix("foaf", FOAF_VOCAB).unwrap();

        // A CURIE with a mapped prefix is expanded correctly.
        assert_eq!(
            mapping.expand_curie_string(curie),
            Ok(String::from("http://xmlns.com/foaf/0.1/Person"))
        );
    }

    #[test]
    fn expand_curie() {
        let mut mapping = PrefixMapping::default();
        mapping.add_prefix("foaf", FOAF_VOCAB).unwrap();

        let curie = Curie::new("foaf", "Agent");
        assert_eq!(
            mapping.expand_curie(&curie),
            Ok(String::from("http://xmlns.com/foaf/0.1/Agent"))
        );
    }
}
