use crate::types::{
    alias::{Price, Size},
    base::{Codes, Direction, Method, Side},
    order::Order,
};
use anyhow::Result;

pub trait Runtime: Send + Sync {
    fn run(&self) -> Result<()>;
    #[allow(clippy::too_many_arguments)]
    fn place_order(
        &self,
        code: Codes,
        method: Method,
        direction: Direction,
        side: Side,
        size: Size,
        price: Price,
        remark: Option<String>,
    ) -> Result<Order>;
    fn cancel_order(&self, code: Codes, id: &str) -> Result<()>;
    fn set_lever(&self, code: Codes, lever: Size) -> Result<()>;
}
