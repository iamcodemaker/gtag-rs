# gtag-js

A wrapper around Google's [gtag.js] framework.

## Example

Ensure the [gtag.js] snippet has been properly installed before your wasm
module runs. Instructions can be found in the [gtag.js documentation].

`Cargo.toml`
```toml
[dependencies]
gtag-js = "0.2"
serde_json = "1.0"
```


`lib.rs`
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

# gtm-js

A wrapper around Google's [gtm.js] snippet.

## Example

Ensure the [gtm.js] snippet has been properly installed before your wasm
module runs. Instructions can be found in the [gtm.js documentation].

`Cargo.toml`
```toml
[dependencies]
gtm-js = "0.1"
serde_json = "1.0"
```


`lib.rs`
```rust
use serde_json::json;

fn main() {
    gtm_js::push("config", json!({
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

[gtag.js]: https://developers.google.com/gtagjs
[gtag.js documentation]: https://developers.google.com/gtagjs/devguide/snippet
[gtm.js]: https://developers.google.com/tag-manager/quickstart
[gtm.js documentation]: https://developers.google.com/tag-manager/quickstart
