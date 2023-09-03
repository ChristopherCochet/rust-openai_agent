// web micro service for calculating multiple types of matehmatical expressions - add, minus, multiply, divide

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a calculator web micro service")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::add(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(add))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
