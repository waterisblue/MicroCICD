use crate::config::CONFIG;
use crate::server::start;

mod server;
mod config;
mod sh;

#[tokio::main]
async fn main() {
    let port = CONFIG.server.port;
    start("0.0.0.0", port).await.unwrap();
}
