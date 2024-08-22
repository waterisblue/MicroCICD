use crate::sh::exec;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/task/{name}")]
async fn greet(task: web::Path<String>) -> impl Responder {
    let output = exec(task.as_str()).await;
    output
}

pub async fn start(ip: &str, port: i32) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
        .bind((format!("{ip}:{port}")))?
        .run()
        .await
}