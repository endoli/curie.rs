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
//! Example CURIEs:
//!
//! * `"foaf:Person"` -- Results in a URI in the `foaf` namespace.
//! * `":Person"` -- Results in a URI in the default namespace.
//! * `"Person"` -- Results in a URI in the default namespace.
//!
//! The last two examples rely upon there being a default mapping providing
//! a default base URI.
//!
//! See the [specification] for further details.
//!
//! ## Usage
//!
//! ```
//! use curie::PrefixMapping;
//!
//! // Initialize a prefix mapper.
//! let mut mapper = PrefixMapping::default();
//! mapper.add_prefix("foaf", "http://xmlns.com/foaf/0.1/");
//!
//! // Set a default prefix
//! mapper.set_default("http://example.com/");
//!
//! // Expand a CURIE and get back the full URI.
//! assert_eq!(mapper.expand("Entity"),
//!            Ok(String::from("http://example.com/Entity")));
//! assert_eq!(mapper.expand("foaf:Agent"),
//!            Ok(String::from("http://xmlns.com/foaf/0.1/Agent")));
//! ```
//!
//! [defined by the W3C]: https://www.w3.org/TR/curie/
//! [specification]: https://www.w3.org/TR/curie/

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

use std::collections::HashMap;

/// Errors that might occur during CURIE expansion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrefixMappingError {
    /// The prefix on the CURIE has no valid mapping.
    Invalid,
    /// The CURIE uses a default prefix, but one has not
    /// been set.
    MissingDefault,
}

/// Maps prefixes to base URIs and allows for the expansion of
/// CURIEs (Compact URIs).
#[derive(Default)]
pub struct PrefixMapping<'pm> {
    default: Option<&'pm str>,
    mapping: HashMap<&'pm str, &'pm str>,
}

impl<'pm> PrefixMapping<'pm> {
    /// Set a default prefix.
    ///
    /// This is used during CURIE expansion when there is no
    /// prefix, just a reference value.
    pub fn set_default(&mut self, default: &'pm str) {
        self.default = Some(default)
    }

    /// Add a prefix to the mapping.
    ///
    /// This allows this prefix to be resolved when `expand` is
    /// invoked on a CURIE.
    pub fn add_prefix(&mut self, prefix: &'pm str, value: &'pm str) {
        self.mapping.insert(prefix, value);
    }

    /// Remove a prefix from the mapping.
    ///
    /// Future calls to `expand` that use this `prefix` will result
    /// in a `PrefixMappingError::Invalid` error.
    pub fn remove_prefix(&mut self, prefix: &str) {
        self.mapping.remove(prefix);
    }

    /// Expand a CURIE, returning a complete IRI.
    pub fn expand(&self, curie: &str) -> Result<String, PrefixMappingError> {
        if let Some(separator_idx) = curie.chars().position(|c| c == ':') {
            let prefix = &curie[..separator_idx];
            let reference = &curie[separator_idx + 1..];

            // If we have a separator, try to expand.
            if separator_idx > 0 {
                if let Some(mapped_prefix) = self.mapping.get(prefix) {
                    Ok(String::from(*mapped_prefix) + reference)
                } else {
                    Err(PrefixMappingError::Invalid)
                }
            } else {
                // Separator was first character, so look for default.
                // No separator, so look for default.
                if let Some(default) = self.default {
                    Ok(String::from(default) + reference)
                } else {
                    Err(PrefixMappingError::MissingDefault)
                }
            }
        } else {
            // No separator, so look for default.
            if let Some(default) = self.default {
                Ok(String::from(default) + curie)
            } else {
                Err(PrefixMappingError::MissingDefault)
            }
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
        pm.add_prefix("foaf", FOAF_VOCAB);
        assert_eq!(pm.mapping.get("foaf"), Some(&FOAF_VOCAB));

        // Unrelated keys still can not be found.
        assert_eq!(pm.mapping.get("rdfs"), None);

        // Keys can be removed.
        pm.remove_prefix("foaf");

        // The "foaf" key should not be found.
        assert_eq!(pm.mapping.get("foaf"), None);
    }

    #[test]
    fn expand() {
        let mut mapping = PrefixMapping::default();

        let curie = "foaf:Person";

        // A CURIE with an unmapped prefix isn't expanded.
        assert_eq!(mapping.expand(curie), Err(PrefixMappingError::Invalid));

        // A CURIE without a separator doesn't cause problems. It still
        // requires a default though.
        assert_eq!(mapping.expand("Person"),
                   Err(PrefixMappingError::MissingDefault));

        mapping.set_default("http://example.com/");

        assert_eq!(mapping.expand("Person"),
                   Ok(String::from("http://example.com/Person")));

        // Using a colon without a prefix results in using the default.
        assert_eq!(mapping.expand(":Person"),
                   Ok(String::from("http://example.com/Person")));

        // And having a default won't allow a prefixed CURIE to
        // be expanded with the default.
        assert_eq!(mapping.expand(curie), Err(PrefixMappingError::Invalid));

        mapping.add_prefix("foaf", FOAF_VOCAB);

        // A CURIE with a mapped prefix is expanded correctly.
        assert_eq!(mapping.expand(curie),
                   Ok(String::from("http://xmlns.com/foaf/0.1/Person")));
    }
}
