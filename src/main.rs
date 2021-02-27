use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod requestHandler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/again", web::get().to(requestHandler::index2))
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind("207.246.81.152:80")?
    .run()
    .await
}
