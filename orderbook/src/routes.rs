use std::sync::{Arc, Mutex};

use actix_web::{post, web::{Data, Json}, HttpResponse, Responder};

use crate::{input::{CreateOrderInput, DeleteOrder}, orderbook::{self, OrderBook}, output::{CreateOrderResponse, DeleteOrderResponse}};

#[post("/order")]

pub async fn create_order(body: Json<CreateOrderInput>, orderbook: Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
 
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    let mut orderbook = orderbook.lock().unwrap();
    orderbook.create_order(price, quantity, user_id, side);

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("ads"),
    });
}

#[post("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>, orderbook: Data<OrderBook>) -> impl Responder {
    let order_id = body.order_id;

    return HttpResponse::Ok().json(DeleteOrderResponse{
        filled_qty: 0,
        average_price: 33,
    });

}

#[post("/order")]
pub async fn get_depth( orderbook: Data<OrderBook>) -> impl Responder {
    "Hello world"
}
