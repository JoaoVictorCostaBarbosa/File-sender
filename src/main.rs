use crate::router::glb_router;

mod db;
mod dto;
mod handler;
mod model;
mod repository;
mod router;
mod service;

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;
    println!("Banco conectado: {}", pool.size());

    let app = glb_router(pool);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor subiu em {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
