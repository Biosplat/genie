use axum::{response::Html, routing::get, Router};


#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    Html(
        std::fs::read_to_string("frontend/index.html")
        .expect("no index.html exists")
    )
}
