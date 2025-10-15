use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub price: f64,
    pub quantity: f64,
    pub user_id: String,
    pub side: Side,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrder {
    pub order_id: String,
    pub user_id: String,
}