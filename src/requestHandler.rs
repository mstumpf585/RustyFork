use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Yo check it out we made a website wow</h1>")
}

pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
