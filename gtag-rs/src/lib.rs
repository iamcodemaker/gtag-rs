#![ deny(missing_docs)]

//! This crate provides a rustic wrapper around Google's gtag.js tagging framework.
//!
//! To use this crate the `gtag()` javascript function must have already been defined as described
//! in the relevant [documentation].
//!
//! [documentation]: https://developers.google.com/gtagjs/devguide/snippet

use wasm_bindgen::prelude::*;
use serde::ser::Serialize;

/// A wrapper around Google's gtag.js framework.
pub struct DataLayer {
    id: String,
}

impl DataLayer {
    /// Create a `DataLayer` object that can be used to push data with the given ID.
    pub fn new(id: impl Into<String>) -> Self {
        DataLayer {
            id: id.into(),
        }
    }

    /// Push data with no parameters.
    pub fn push_simple(&self, cmd: &str) {
        gtag_js_sys::gtag(cmd, &self.id);
    }

    /// Push data with the given parameters.
    ///
    /// This function can fail if there is an error serializing the data to a [`JsValue`].
    ///
    /// As a parameter value, this accepts any type that can be serialized to Json using Serde.
    /// This can be done by deriving `Serialize` from `serde` or using the `json!` macro from
    /// `serde_json`.
    ///
    /// ```
    /// #[derive(Serialize)]
    /// struct Pageview {
    ///     page_title: Option<String>,
    ///     page_location: Option<String>,
    ///     page_path: Option<String>,
    /// }
    /// ```
    /// ```
    /// json!({
    ///     "page_title": "index.html",
    ///     "page_path": "/",
    /// })
    /// ```
    ///
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/0.2.58/wasm_bindgen/struct.JsValue.html
    pub fn push(&self, cmd: &str, params: &impl Serialize) -> serde_json::error::Result<()> {
        gtag_js_sys::gtag_with_parameters(cmd, &self.id, &JsValue::from_serde(params)?);
        Ok(())
    }
}
