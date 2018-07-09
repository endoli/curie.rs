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
//! so to save time during expansion, the [`Curie`] struct can also be
//! used:
//!
//! ```
//! use curie::{Curie, PrefixMapping};
//!
//! // Initialize a prefix mapper.
//! let mut mapper = PrefixMapping::default();
//! mapper.add_prefix("foaf", "http://xmlns.com/foaf/0.1/").unwrap();
//!
//! let curie = Curie::new(Some("foaf"), "Agent");
//!
//! assert_eq!(mapper.expand_curie(&curie),
//!            Ok(String::from("http://xmlns.com/foaf/0.1/Agent")));
//! ```
//!
//! Given an IRI is also possible to derive an CURIE.
//!
//! ```
//! use curie::{Curie, PrefixMapping};
//!
//! // Initialize a prefix mapper.
//! let mut mapper = PrefixMapping::default();
//! mapper.add_prefix("foaf", "http://xmlns.com/foaf/0.1/").unwrap();
//!
//! let curie = Curie::new(Some("foaf"), "Agent");
//!
//! assert_eq!(Ok(curie),
//!            mapper.shrink_iri("http://xmlns.com/foaf/0.1/Agent"));
//! ```
//!
//! [defined by the W3C]: https://www.w3.org/TR/curie/
//! [specification]: https://www.w3.org/TR/curie/
//! [`Curie`]: struct.Curie.html

#![warn(missing_docs)]
#![deny(
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use std::collections::HashMap;
use std::fmt;

/// Errors that might occur when adding a prefix to a [`PrefixMapping`].
///
/// [`PrefixMapping`]: struct.PrefixMapping.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InvalidPrefixError {
    /// This is a reserved prefix.
    ///
    /// The prefix `"_"` is reserved.
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
///
/// # Examples
///
/// ```
/// use curie::PrefixMapping;
///
/// // Create using the `Default` trait:
/// let mut mapping = PrefixMapping::default();
/// ```
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
    ///
    /// # Example:
    ///
    /// ```
    /// use curie::{ExpansionError, PrefixMapping};
    ///
    /// let mut mapping = PrefixMapping::default();
    ///
    /// // No default has been configured, so an error will be
    /// // signaled.
    /// assert_eq!(mapping.expand_curie_string("Entity"),
    ///            Err(ExpansionError::MissingDefault));
    ///
    /// mapping.set_default("http://example.com/");
    ///
    /// assert_eq!(mapping.expand_curie_string("Entity"),
    ///            Ok(String::from("http://example.com/Entity")));
    /// ```
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
    pub fn expand_curie_string(&self, curie_str: &str) -> Result<String, ExpansionError> {
        if let Some(separator_idx) = curie_str.chars().position(|c| c == ':') {
            // If we have a separator, try to expand.
            let prefix = Some(&curie_str[..separator_idx]);
            let reference = &curie_str[separator_idx + 1..];
            let curie = Curie::new(prefix, reference);
            self.expand_curie(&curie)
        } else {
            let curie = Curie::new(None, curie_str);
            self.expand_curie(&curie)
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
        prefix: Option<&str>,
        reference: &str,
    ) -> Result<String, ExpansionError> {
        if let Some(prefix) = prefix {
            if let Some(mapped_prefix) = self.mapping.get(prefix) {
                Ok((*mapped_prefix).clone() + reference)
            } else {
                Err(ExpansionError::Invalid)
            }
        } else if let Some(ref default) = self.default {
            Ok((default).clone() + reference)
        } else {
            Err(ExpansionError::MissingDefault)
        }
    }

    /// Shrink an IRI returning a [`Curie`]
    ///
    /// [`Curie`]: struct.Curie.html
    pub fn shrink_iri<'a>(&'a self, iri: &'a str) -> Result<Curie<'a>, &'static str> {
        if let Some(ref def) = self.default {
            if iri.starts_with(def) {
                return Ok(Curie::new(None, iri.trim_left_matches(def)));
            }
        }

        for mp in &self.mapping {
            if iri.starts_with(mp.1) {
                return Ok(Curie::new(Some(mp.0), iri.trim_left_matches(mp.1)));
            }
        }

        Err("Unable to shorten")
    }

    /// Return an iterator over the prefix mappings.
    ///
    /// This is useful when testing code that uses this crate.
    pub fn mappings(&self) -> ::std::collections::hash_map::Iter<String, String> {
        self.mapping.iter()
    }
}

/// A prefix and reference, already parsed into separate components.
///
/// When parsing a document, the components of the compact URI will already
/// have been parsed and we can avoid storing a string of the full compact
/// URI and having to do that work again when expanding the compact URI.
///
/// The `'c` lifetime parameter will typically be the lifetime of the body
/// of text which is being parsed and contains the compact URIs.
///
/// # Usage:
///
/// ## Creation:
///
/// ```
/// # use curie::Curie;
/// let c = Curie::new(Some("foaf"), "Person");
/// ```
///
/// ## Expansion:
///
/// Expanding a `Curie` requires the use of a properly initialized
/// [`PrefixMapping`].
///
/// ```
/// # use curie::{Curie, PrefixMapping};
/// // Initialize a prefix mapper.
/// let mut mapper = PrefixMapping::default();
/// mapper.add_prefix("foaf", "http://xmlns.com/foaf/0.1/").unwrap();
///
/// let curie = Curie::new(Some("foaf"), "Agent");
///
/// assert_eq!(mapper.expand_curie(&curie),
///            Ok(String::from("http://xmlns.com/foaf/0.1/Agent")));
/// ```
///
/// ## Display / Formatting:
///
/// `Curie` implements the `Debug` and `Display` traits, so it integrates with
/// the Rust standard library facilities.
///
/// ```
/// # use curie::Curie;
/// let curie = Curie::new(Some("foaf"), "Agent");
/// assert_eq!("foaf:Agent", format!("{}", curie));
/// ```
///
/// [`PrefixMapping`]: struct.PrefixMapping.html
#[derive(Debug, Eq, PartialEq)]
pub struct Curie<'c> {
    prefix: Option<&'c str>,
    reference: &'c str,
}

impl<'c> Curie<'c> {
    /// Construct a `Curie` from a prefix and reference.
    pub fn new(prefix: Option<&'c str>, reference: &'c str) -> Self {
        Curie { prefix, reference }
    }
}

impl<'c> From<&'c Curie<'c>> for String {
    fn from(c: &'c Curie<'c>) -> String {
        format!("{}", c)
    }
}

impl<'c> From<Curie<'c>> for String {
    fn from(c: Curie<'c>) -> String {
        format!("{}", c)
    }
}

impl<'c> fmt::Display for Curie<'c> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.prefix {
            Some(prefix) => write!(f, "{}:{}", prefix, self.reference),
            None => write!(f, "{}", self.reference),
        }
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
        let curie = Curie::new(Some("foaf"), "Agent");
        assert_eq!("foaf:Agent", format!("{}", curie));
    }

    #[test]
    fn from_string_curie() {
        let curie = Curie::new(Some("foaf"), "Agent");
        assert_eq!("foaf:Agent", String::from(curie));

        let curie = Curie::new(None, "Agent");
        assert_eq!("Agent", String::from(curie));

        let curie = Curie::new(Some("foaf"), "Agent");
        assert_eq!("foaf:Agent", String::from(&curie));
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

        let curie = Curie::new(Some("foaf"), "Agent");
        assert_eq!(
            mapping.expand_curie(&curie),
            Ok(String::from("http://xmlns.com/foaf/0.1/Agent"))
        );
    }

    #[test]
    fn expand_curie_default() {
        let mut mapping = PrefixMapping::default();
        mapping.set_default(FOAF_VOCAB);

        let curie = Curie::new(None, "Agent");
        assert_eq!(
            mapping.expand_curie(&curie),
            Ok(String::from("http://xmlns.com/foaf/0.1/Agent"))
        );
    }

    #[test]
    fn shrink_iri_prefix() {
        let mut mapping = PrefixMapping::default();
        mapping.add_prefix("foaf", FOAF_VOCAB).unwrap();

        let curie = Curie::new(Some("foaf"), "Agent");

        assert_eq!(
            mapping.shrink_iri("http://xmlns.com/foaf/0.1/Agent"),
            Ok(curie)
        );
    }

    #[test]
    fn split_iri_default() {
        let mut mapping = PrefixMapping::default();
        mapping.set_default(FOAF_VOCAB);

        let curie = Curie::new(None, "Agent");

        assert_eq!(
            mapping.shrink_iri("http://xmlns.com/foaf/0.1/Agent"),
            Ok(curie)
        );
    }

}
