use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::env;
use tera::{Tera}; 
pub mod request_handler;

/*
* This is the main entry point 

*/
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let address = &args[1];

    println!("starting server at {}", address);
    HttpServer::new(|| {

        // Put our generator to the folder location of the templates. 
        let tera = Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/template/**/*")
        ).unwrap(); 

        App::new()
            .data(request_handler::AppData {tmpl: tera})
            .route("/hello", web::get().to(request_handler::index)) // different routes 
            .route("/again", web::get().to(request_handler::index2)) 
            .service(request_handler::index3) // not sure whats going on with this
            .service(web::resource("tmpl/{name}").route(web::get().to(request_handler::index4))) // use of template examples 
            .service(Files::new("/", "./static/root/").index_file("index.html")) // server up a static page 

    })
    .bind(address)?
    .run()
    .await
}
