use poem::{handler, Route, Server, Endpoint};
use poem::listener::TcpListener;
use poem::web::Path;
use crate::sh::exec;

#[handler]
async fn post_task(Path(task): Path<String>) -> String {
    let output = exec(&task).await.unwrap_or_else(|e| format!("Error executing task:{}", e));
    output
}

pub async fn start(ip: &str, port: &str) -> std::io::Result<()> {
    let app = Route::new().at("/task/:task", post_task);

    let address = format!("{}:{}", ip, port);
    println!("Listening on http://{}", address);

    // 创建 TcpListener 并绑定地址
    let listener = TcpListener::bind(&address);

    // 创建 Server 并运行
    Server::new(listener)
        .run(app)
        .await?;

    Ok(())
}
