# axum-wasm-macros

[![Crates.io](https://img.shields.io/crates/v/axum-wasm-macros)](https://crates.io/crates/axum-wasm-macros)


A macro to ensure async axum routes can compile to WASM

## Why? 

Axum handlers return a `Send` future. However, JS types do not return a `Send` future. `wasm_compat` will provide compatability between the return types.


## Example

```rust
use axum_wasm_macros::wasm_compat;
use axum::Router;
use axum::routing::get;

#[wasm_compat]
pub async fn index() -> &'static str {
    "Hello World"
}

pub fn main() {
    let router: Router = Router::new().route("/", get(index));
    // rest of the app code goes here.
}
```