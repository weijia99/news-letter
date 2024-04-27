use std::net::TcpListener;

use actix_web::web::Form;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;


#[derive(serde::Deserialize)]
struct FormData{
    email: String,
    name: String,

}
async fn health_check() ->  HttpResponse{
    HttpResponse::Ok().finish()
}


// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
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

async fn subscribe(_form:Form<FormData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}