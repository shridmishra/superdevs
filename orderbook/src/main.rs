use std::sync::{Arc, Mutex};

use actix_web::{web::{ Data}, App, HttpServer};

use crate::orderbook::Orderbook;

pub mod routes;
pub mod input;
pub mod output;
pub mod orderbook;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orderbook = Arc::new(Mutex::new(Orderbook::default()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(orderbook.clone()))
            .service(routes::create_order)
            .service(routes::delete_order)
            .service(routes::get_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
