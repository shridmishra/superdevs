use actix_web::{HttpResponse, Responder, post, web::Json};

use crate::{input::{CreateOrderInput, DeleteOrder}, output::{CreateOrderResponse, DeleteOrderResponse}};

#[post("/order")]

pub async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("ads"),
    });
}

#[post("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>) -> impl Responder {
    let order_id = body.order_id;

    return HttpResponse::Ok().json(DeleteOrderResponse{
        filled_qty: 0,
        average_price: 33,
    });

}

#[post("/order")]
pub async fn get_depth() -> impl Responder {
    "Hello world"
}
