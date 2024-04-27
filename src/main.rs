use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn health_check() ->  HttpResponse{
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod test{
    use crate::health_check;
    #[tokio::test]
    async fn health_check_succeeds(){
        let response =health_check().await;
        assert!(response.status().is_success());
    }
}