use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Responder};
use actix_web::get;
use tera::{Tera, Context}; 

pub struct AppData {
    pub tmpl: Tera
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Yo check it out we made a website wow</h1>")
}

pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[get("/hello")]
pub async fn index3() -> impl Responder {
    HttpResponse::Ok().body("hey there!")
}

pub async fn index4(data: web::Data<AppData>, req:HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", name);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}           
