# gtag-js [![Crates.io](https://img.shields.io/crates/v/gtag-js.svg)](https://crates.io/crates/gtag-js)
 

A rustic wrapper around Google's [gtag.js] javascript framework. See the
`gtag-js-sys` crate for a more low level wrapper.

## Example

Ensure the [gtag.js] snippet has been properly installed before your wasm
module runs. Instructions can be found in the [gtag.js documentation].

```rust
use gtag_js::DataLayer;
use serde_json::json;

fn main() {
    let gtag = DataLayer::new("GA_MEASUREMENT_ID");

    gtag.push("config", json!({
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

[gtag.js]: https://developers.google.com/gtagjs
[gtag.js documentation]: https://developers.google.com/gtagjs/devguide/snippet
