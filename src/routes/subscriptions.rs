

use actix_web::web::Form;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use sqlx::{PgConnection, PgPool};

use chrono::Utc;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData{
    email: String,
    name: String,

}


// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.


pub async fn subscribe(form:Form<FormData>,connection:web::Data<PgPool>) -> HttpResponse{
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id,email,name,subscribed_at) VALUES ($1,$2,$3,$4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // use get_ref() to get a reference to the connection
    .execute(connection.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}