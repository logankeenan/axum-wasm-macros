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

#[wasm_compat]
async fn test_route() -> impl IntoResponse {
    let text = reqwest::get("https://logankeenan.com/").await.unwrap().text().await.unwrap();

    (StatusCode::OK, text)
}

pub async fn make_request() -> Response {
    let mut route = Router::new()
        .route("/", get(test_route));

    let request = Request::get("/").body(Body::empty()).unwrap();
    let response = route.call(request).await.unwrap();


    response
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn it_should_return_a_200() {
        let response = make_request().await;
        assert_eq!(response.status(), 200);
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
        let response = make_request().await;
        assert_eq!(response.status(), 200);
    }
}
