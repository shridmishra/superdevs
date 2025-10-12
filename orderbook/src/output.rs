use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderResponse {
    pub order_id: String,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct DeleteOrderResponse {
    pub filled_qty: u32,
    pub average_price: u32,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Depth {
    pub bids: Vec<[u32; 2]>,
    pub ask: Vec<[u32; 2]>,
    pub last_update_id: String,
}
