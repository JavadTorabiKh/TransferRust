use actix_web::{web, App, HttpServer};
use actix_web::dev::Service;
use futures::future;
mod network_apis;
use crate::bearer::Bearers;
mod bearer;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let my_config = Bearers::new();

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/v1/sui/balance/{address}")
                    .route(web::get().to(network_apis::get_balance))
            )
            })
            
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

