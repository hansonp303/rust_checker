use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use crate::report::{ValidationSummary, FileValidationResult};
use std::sync::Mutex;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(" Rust Checker Web Dashboard is running!")
}

#[get("/summary")]
async fn summary(data: web::Data<Mutex<ValidationSummary>>) -> impl Responder {
    let summary = data.lock().unwrap();
    HttpResponse::Ok().json(&*summary)
}

pub async fn run_dashboard(summary: ValidationSummary) -> std::io::Result<()> {
    let shared_data = web::Data::new(Mutex::new(summary));
    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(index)
            .service(summary)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

