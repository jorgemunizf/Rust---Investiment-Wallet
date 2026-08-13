mod db;
mod handlers;
mod routes;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não encontrada");

    let pool = db::create_pool(&database_url).await;

    println!("Conectado ao PostgreSQL!");

    let app = routes::create_routes(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Servidor rodando em http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
