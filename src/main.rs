use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::Mutex;

use axum_diesel::infra::db::establish_connection;
use axum_diesel::routes::create_routes;

#[tokio::main]
async fn main() {
    let pool = Arc::new(Mutex::new(establish_connection()));
    let app = create_routes(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // log when the server is running
    println!("Server running on port 3000");
    axum::serve(listener, app).await.unwrap();
}
