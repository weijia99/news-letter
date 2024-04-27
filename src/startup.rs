use std::net::TcpListener;
use crate::routes::{health_check, subscribe};

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use sqlx::{PgConnection, PgPool};
pub  fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    // usr arc to share the connection across multiple threads
    // use move to transfer ownership of the connection to the closure
    let server = HttpServer::new(move || {
     App::new()
     .route("/health_check", web::get().to(health_check))
     .route("/subscriptions", web::post().to(subscribe))
     .app_data(connection.clone())
     })
     .listen(listener)?
     .run();
     Ok(server)
     }