mod entity;
mod repository;
mod use_case;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Serialize;

async fn status() -> HttpResponse {
  #[derive(Serialize)]
  struct Response {
    status: String,
  }
  HttpResponse::Ok().json(Response {
    status: String::from("OK!"),
  })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new().route("/status", web::get().to(status)).route(
      "/warehouse/{name}",
      web::get().to(use_case::get_warehouse::http_get_warehouse),
    )
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
