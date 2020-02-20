#![ deny(missing_docs)]

//! This crate provides a wrapper around Google's gtag.js tagging framework.
//!
//! To use this crate the `gtag()` javascript function must have already been defined as described
//! in the relevant [documentation].
//!
//! [documentation]: https://developers.google.com/gtagjs/devguide/snippet

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    /// Send event data.
    #[wasm_bindgen(js_name = gtag)]
    pub fn gtag(cmd: &str, id: &str);

    /// Send event data with parameters.
    #[wasm_bindgen(js_name = gtag)]
    pub fn gtag_with_parameters(cmd: &str, id: &str, params: &JsValue);
}
