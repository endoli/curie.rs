// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CURIE: Compact URIs
//!
//! CURIE, [defined by the W3C], are a compact way of representing a URI.
//!
//! [defined by the W3C]: https://www.w3.org/TR/curie/

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

use std::collections::HashMap;

#[allow(missing_docs)]
#[derive(Default)]
pub struct PrefixMapping<'pm> {
    mapping: HashMap<&'pm str, &'pm str>,
}

#[allow(missing_docs)]
impl<'pm> PrefixMapping<'pm> {
    pub fn add_prefix(&mut self, prefix: &'pm str, value: &'pm str) {
        self.mapping.insert(prefix, value);
    }

    pub fn remove_prefix(&mut self, prefix: &str) {
        self.mapping.remove(prefix);
    }

    pub fn get_prefix_value(&self, prefix: &str) -> Option<&&str> {
        self.mapping.get(prefix)
    }

    pub fn get_prefix_for_value(&self, value: &str) -> Option<&&str> {
        self.mapping.iter().find(|&(_, v)| *v == value).map(|(k, _)| k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_remove_works() {
        let mut mapping = PrefixMapping::default();

        const FOAF_VOCAB: &'static str = "http://xmlns.com/foaf/0.1/";

        // No keys should be found.
        assert_eq!(mapping.get_prefix_value("foaf"), None);
        assert_eq!(mapping.get_prefix_for_value("foaf"), None);

        // Add and look up a key.
        mapping.add_prefix("foaf", FOAF_VOCAB);
        assert_eq!(mapping.get_prefix_value("foaf"), Some(&FOAF_VOCAB));
        assert_eq!(mapping.get_prefix_for_value(FOAF_VOCAB), Some(&"foaf"));

        // Unrelated keys still can not be found.
        assert_eq!(mapping.get_prefix_value("rdfs"), None);
        assert_eq!(mapping.get_prefix_for_value("rdfs"), None);

        // Keys can be removed.
        mapping.remove_prefix("foaf");

        // The "foaf" key should be found.
        assert_eq!(mapping.get_prefix_value("foaf"), None);
        assert_eq!(mapping.get_prefix_for_value("foaf"), None);
    }
}
