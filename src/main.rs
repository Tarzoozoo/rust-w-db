mod server;
mod app;

#[tokio::main]
async fn main() {
    server::start().await;
}