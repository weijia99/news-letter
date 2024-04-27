use std::net::TcpListener;

use news::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
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