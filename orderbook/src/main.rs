use actix_web::{App, HttpResponse, HttpServer, Responder, post, web::Json};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .service(create_order)
            .service(get_order)
            .service(get_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Serialize, Deserialize)]
struct Rect {
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateOrderInput {
    price: u32,
    quantity: u32,
    user_id: u32,
    side: Side,
}
#[derive(Serialize, Deserialize, Debug)]
enum Side {
    Buy,
    Sell,
}

#[post("/order")]
async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    println!("{:?}", body);

    let r = Rect {
        width: 35,
        height: 55,
    };
    return HttpResponse::Ok().json(r);
}

#[post("/order")]
async fn get_order() -> impl Responder {
    "Hello world"
}

#[post("/order")]
async fn get_depth() -> impl Responder {
    "Hello world"
}
