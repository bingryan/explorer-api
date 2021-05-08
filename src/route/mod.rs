use actix_web::{web, HttpResponse, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .set_header("Access-Control-Allow-Origin", "*")
        .set_header("Cache-Control", "no-cache")
        .body("Hello World!")
}


pub fn values(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").to(index));
}