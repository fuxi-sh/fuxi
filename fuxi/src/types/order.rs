use crate::types::{
    alias::{Price, Size, Time},
    base::{Codes, Direction, Method, OrderStatus, Side},
};
use fuxi_macros::{define_map, model};

#[model(python)]
pub struct Order {
    pub code: Codes,
    pub id: String,
    pub method: Method,
    pub direction: Direction,
    pub side: Side,
    pub status: OrderStatus,
    pub size: Size,
    pub price: Price,
    pub deal_size: Size,
    pub deal_price: Price,
    pub deal_fee: Size,
    pub margin: Size,
    pub remark: Option<String>,
    pub create_time: Time,
    pub update_time: Time,
}

define_map!(pub OrderMap is String to Order);

impl OrderMap {
    #[inline]
    pub fn remove_expired(&self) {
        self.maps_mut().retain(|_, order| {
            matches!(
                *order.status(),
                OrderStatus::Rejected | OrderStatus::Completed | OrderStatus::Cancelled
            )
        });
    }
}
