use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod requestHandler;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(requestHandler::index))
            .route("/again", web::get().to(requestHandler::index2))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
