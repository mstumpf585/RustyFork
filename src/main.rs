use actix_files::Files;
use actix_web::{web, App, HttpServer};
use tera::{Tera, Context}; 
pub mod requestHandler;

/*
* This is the main entry point 

*/
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {

        // Put our generator to the folder location of the templates. 
        let tera = Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/template/**/*")
        ).unwrap(); 

        App::new()
            .data(requestHandler::AppData {tmpl: tera})
            .route("/hello", web::get().to(requestHandler::index)) // different routes 
            .route("/again", web::get().to(requestHandler::index2)) 
            .service(requestHandler::index3) // not sure whats going on with this
            .service(web::resource("tmpl/{name}").route(web::get().to(requestHandler::index4))) // use of template examples 
            .service(Files::new("/", "./static/root/").index_file("index.html")) // server up a static page 

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
