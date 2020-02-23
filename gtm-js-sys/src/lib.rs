#![ deny(missing_docs)]

//! This crate provides a wrapper around Google's gtm.js' `dataLayer.push()` function.
//!
//! To use this crate the tag manager must have already been imported as described in the relevant
//! [documentation].
//!
//! [documentation]: https://developers.google.com/tag-manager/quickstart

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    /// Push tag data (aka the `dataLayer.push()` function).
    #[wasm_bindgen(js_namespace = dataLayer)]
    pub fn push(vars: &JsValue);
}
