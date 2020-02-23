#![ deny(missing_docs)]

//! This crate provides a rustic wrapper around Google's gtm.js tag manager snippet.
//!
//! To use this crate the tag manager must have already been imported as described in the relevant
//! [documentation].
//!
//! [documentation]: https://developers.google.com/tag-manager/quickstart

use wasm_bindgen::prelude::*;
use serde::ser::Serialize;

/// Push variables and/or events.
///
/// This function can fail if there is an error serializing the data to a [`JsValue`].
///
/// As a parameter value, this accepts any type that can be serialized to Json using Serde.  This
/// can be done by deriving `Serialize` from `serde` or using the `json!` macro from `serde_json`.
///
/// ```
/// #[derive(Serialize)]
/// struct PageviewEvent {
///     event: String,
///     page_path: String,
/// }
///
/// impl PageviewEvent {
///     fn new(path: String) -> Self {
///         PageviewEvent {
///             event: "custom-page-view",
///             page_path: path,
///         }
///     }
/// }
/// ```
/// ```
/// json!({
///     "event": "custom-page-view",
///     "page_path": "/path",
/// })
/// ```
///
/// [`JsValue`]: https://docs.rs/wasm-bindgen/0.2.58/wasm_bindgen/struct.JsValue.html
pub fn push(vars: &impl Serialize) -> serde_json::error::Result<()> {
    gtm_js_sys::push(&JsValue::from_serde(vars)?);
    Ok(())
}
