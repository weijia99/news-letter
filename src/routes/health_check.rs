
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
pub async fn health_check() ->  HttpResponse{
    HttpResponse::Ok().finish()
}
