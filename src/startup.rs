use std::net::TcpListener;
use crate::routes::{health_check, subscribe};

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
pub  fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
     App::new()
     .route("/health_check", web::get().to(health_check))
     .route("/subscriptions", web::post().to(subscribe))
     })
     .listen(listener)?
     .run();
     Ok(server)
     }