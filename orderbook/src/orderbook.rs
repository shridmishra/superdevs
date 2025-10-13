use std::collections::HashMap;

use crate::input::Side;

pub struct OrderBook {
    pub bids: HashMap<u32, Vec<UserOrder>>,
    pub asks: HashMap<u32, Vec<UserOrder>>,
    order_id_index: u32,
}

pub struct UserOrder {
    pub user_id: u32,
    pub qty: u32,
    pub order_id: u32,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: HashMap::new(),
            asks: HashMap::new(),
            order_id_index:0,
        }
    }
}

impl OrderBook {
    pub fn create_order(&mut self, price: u32, quantity: u32, user_id: u32, side: Side) {
        let order_id = self.order_id_index;
        self.order_id_index = self.order_id_index +  1;
        if side == Side::Buy {
            self.bids
                .entry(price)
                .or_insert(Vec::new())
                .push(UserOrder {
                    user_id,
                    qty: quantity,
                    order_id: order_id,
                });
        } else {
            self.asks
                .entry(price)
                .or_insert(Vec::new())
                .push(UserOrder {
                    user_id,
                    qty: quantity,
                    order_id: order_id,
                });
        }
    }

    pub fn get_depth(&self) -> &OrderBook {
        self
    }
   }
