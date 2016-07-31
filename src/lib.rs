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
pub struct PrefixMapping<'pm> {
    mapping: HashMap<&'pm str, &'pm str>,
}

#[allow(missing_docs)]
impl<'pm> PrefixMapping<'pm> {
    pub fn add_ns_prefix(&mut self, prefix: &'pm str, uri: &'pm str) {
        self.mapping.insert(prefix, uri);
    }

    pub fn remove_ns_prefix(&mut self, prefix: &str) {
        self.mapping.remove(prefix);
    }

    pub fn get_ns_prefix_uri(&self, prefix: &str) -> Option<&&str> {
        self.mapping.get(prefix)
    }

    pub fn get_ns_uri_prefix(&self, _uri: &str) -> Option<&&str> {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
