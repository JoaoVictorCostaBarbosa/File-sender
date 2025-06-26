mod db;

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;
    println!("Banco conectado: {}", pool.size());
}
