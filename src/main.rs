use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Servidor rodando em http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Investiment Wallet"
}
