use axum::{
    extract::Query,
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "GET!\n" }))
        .route("/", post(|| async { "POST!\n" }))
        .route("/data", get(data))
        .route("/data", post(data));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serde::Deserialize)]
struct Size {
    size: u16,
}

async fn data(size: Query<Size>) -> String {
    let size = size.0.size as usize;
    "b".to_owned().repeat(size)
}
