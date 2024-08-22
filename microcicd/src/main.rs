use crate::config::CONFIG;
use crate::server::start;

mod server;
mod config;
mod sh;

#[tokio::main]
async fn main() {
    let default_port = "7777".to_string();
    let port = CONFIG.get("port").unwrap_or(&default_port);
    start("0.0.0.0", port).await.unwrap();
}
