use std::net::TcpListener;

use news::{configuration::{self, get_configuration}, startup::run};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}",configuration.application_port);

    let listener = TcpListener::bind(address)
.expect("Failed to bind random port");

    run(listener)?.await
}
  

// #[cfg(test)]
// mod test{
//     use crate::health_check;
//     #[tokio::test]
//     async fn health_check_succeeds(){
//         let response =health_check().await;
//         assert!(response.status().is_success());
//     }
// }