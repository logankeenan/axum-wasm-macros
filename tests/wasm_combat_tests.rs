use axum_wasm_macros::wasm_compat;
use axum::{
    body::Body,
    http::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
    routing::get
};
use tower_service::Service;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomError;

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error occurred")
    }
}

impl Error for CustomError {}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}


#[wasm_compat]
async fn test_route() -> impl IntoResponse {
    let text = reqwest::get("https://logankeenan.com/").await.unwrap().text().await.unwrap();

    (StatusCode::OK, text)
}

#[wasm_compat]
async fn test_route_with_result() -> Result<impl IntoResponse, CustomError> {
    let text = reqwest::get("https://logankeenan.com/").await.unwrap().text().await.unwrap();

    Ok((StatusCode::OK, text))
}

fn fallible() -> Result<(), CustomError> {
    Err(CustomError)
}

#[wasm_compat]
async fn test_route_with_try_operator() -> Result<impl IntoResponse, CustomError> {
    // in order for these errors to propegate you'd need to wrap them into CustomError with
    // something like the 'thiserror' crate. for simplicity, we just illustrate with another func
    let text = reqwest::get("https://logankeenan.com/").await.unwrap().text().await.unwrap();

    fallible()?;

    Ok((StatusCode::OK, text))
}

pub async fn make_request(path: &str) -> Response {
    let mut route = Router::new()
        .route("/", get(test_route))
        .route("/results", get(test_route_with_result))
        .route("/results_with_try", get(test_route_with_try_operator));

    let request = Request::get(path).body(Body::empty()).unwrap();
    route.call(request).await.unwrap()
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_should_return_a_200() {
        let response = make_request("/").await;
        assert_eq!(response.status(), 200);

        let response = make_request("/results").await;
        assert_eq!(response.status(), 200);

        let response = make_request("/results_with_try").await;
        assert_eq!(response.status(), 500);
    }

}

#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use super::*;
    use wasm_bindgen_test::{*};

    wasm_bindgen_test_configure!(run_in_browser);


    #[wasm_bindgen_test]
    async fn it_should_return_a_200() {
        let response = make_request("/").await;
        assert_eq!(response.status(), 200);

        let response = make_request("/results").await;
        assert_eq!(response.status(), 200);

        let response = make_request("/results_with_try").await;
        assert_eq!(response.status(), 500);
    }
}
