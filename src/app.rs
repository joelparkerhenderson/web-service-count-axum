/// Use axum capabilities.
use axum::routing::get;

/// Create our application by creating our router.
pub fn app() -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .route("/count", get(count))
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, uri.to_string())
}

/// Create the atomic variable COUNT so the program can track its own count.
pub static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// axum handler for "GET /count" which shows the program's count duration.
/// This shows how to write a handler that uses a global static lazy value.
pub async fn count() -> String {
    COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("{}", COUNT.load(std::sync::atomic::Ordering::SeqCst))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test() {
        let app: axum::Router = app();
        let server = TestServer::new(app).unwrap();
        let response_text_0 = server.get("/count").await.text();
        let response_text_1 = server.get("/count").await.text();
        assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1);
    }

}