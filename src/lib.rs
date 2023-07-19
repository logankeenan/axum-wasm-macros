//! Axum WASM Macros
//!
//! Axum handlers return a `Send` future. However, JS types do not return a `Send`
//! future. `wasm_compat` will provide compatability between the return types.
//!
//!
//! ```
//! use axum_wasm_macros::wasm_compat;
//! use axum::Router;
//! use axum::routing::get;
//!
//! #[wasm_compat]
//! pub async fn index() -> &'static str {
//!     "Hello World"
//! }
//!
//! pub fn main() {
//!     let router: Router = Router::new().route("/", get(index));
//!     // rest of the app code goes here.
//! }
//!
//!```

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn wasm_compat(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    let stream_clone = stream.clone();
    let input = parse_macro_input!(stream_clone as ItemFn);

    let ItemFn { attrs, vis, sig, block } = input;
    let stmts = &block.stmts;

    let wasm_result = quote! {
        #(#attrs)* #vis #sig {
            let (tx, rx) = oneshot::channel();
            wasm_bindgen_futures::spawn_local(async move {
                let result = {
                    #(#stmts)*
                };
                tx.send(result).unwrap();
            });

            rx.await.unwrap()
        }
    };

    let not_wasm_result = quote! {
        #(#attrs)* #vis #sig {
            #(#stmts)*
        }
    };

    let result = quote! {
        #[cfg(target_arch = "wasm32")]
        #wasm_result

        #[cfg(not(target_arch = "wasm32"))]
        #not_wasm_result
    };

    TokenStream::from(result)
}

