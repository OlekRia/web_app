use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use tokio;

mod cli;
mod json_serialization;
mod processes;
mod state;
mod to_do;
mod views;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(views::view_factory)
            .route("/", web::get().to(greet))
            .route("/name", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .bind("127.0.0.1:8081")?
    // .workers(3)
    .run()
    .await
}
