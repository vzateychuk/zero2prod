//! lib.rs

use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Notice the different signature!
// We return`Server`on the happy path and we dropped the`async`keyword
// We have no .await call, so it is not needed anymore.
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
