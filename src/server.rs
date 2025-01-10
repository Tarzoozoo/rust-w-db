// Axum web server
use std::net::SocketAddr;
use axum::{
    response::{IntoResponse, Json}, routing::{get, post, put, delete}, Router
};
use serde_json::{json};
use tokio_postgres::NoTls;

pub mod products;

#[tokio::main]
pub async fn start() {

    // Connection parameters
    let host = "localhost";
    let port = 5432;
    let user = "myuser";
    let password = "mypassword";
    let dbname = "mydatabase";

    // Connect to the database
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        host, port, user, password, dbname
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let db_client = std::sync::Arc::new(client);

    let ex = Router::new()
        .route("/:product_id", get(products::handlers::find_one_product));

    let robot = Router::new()
        .route("/", post(products::handlers::add_robot)
                    .get(products::handlers::get_robots))
        .route("/:serial", get(products::handlers::get_robot)
                        .put(products::handlers::update_robot)
                        .delete(products::handlers::delete_robot));
    

    // Combine all routes into the main router
    let app = Router::new()
        .nest("/products", ex) // Nest `products` routes under /products
        .nest("/robot", robot)     // Nest `users` routes under /users
        .with_state(db_client); // Add shared state

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("server is running on -> {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}