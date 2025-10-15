use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub average_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrderResponse {
    pub success: bool,
    pub remaining_quantity: f64,
    pub filled_quantity: f64,
    pub average_price: f64,
}