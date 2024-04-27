

use actix_web::web::Form;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};


#[derive(serde::Deserialize)]
pub struct FormData{
    email: String,
    name: String,

}


// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.


pub async fn subscribe(_form:Form<FormData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}