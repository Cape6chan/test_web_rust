use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler_home))
        .route("/api/ping", get(handler_api));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is listening on: 0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}


async fn handler_home() -> &'static str {
    "Bienvenue dans mon home"
}

async fn handler_api() -> String {
    format!("Ping reçu à {:?}", std::time::SystemTime::now())
}