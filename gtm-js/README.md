# gtm-js [![Crates.io](https://img.shields.io/crates/v/gtm-js.svg)](https://crates.io/crates/gtm-js)


A rustic wrapper around Google's [gtm.js] tag manager. See the `gtm-js-sys`
crate for a more low level wrapper.

## Example

Ensure the [gtm.js] snippet has been properly installed before your wasm
module runs. Instructions can be found in the [gtm.js documentation].

```rust
use serde_json::json;

fn main() {
    gtm_js::push(json!({
        "event": "pageview",
        "page_title": "homepage",
        "page_path": "/home",
    }).unwrap();
}
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[gtm.js]: https://developers.google.com/tag-manager/quickstart
[gtm.js documentation]: https://developers.google.com/tag-manager/quickstart
