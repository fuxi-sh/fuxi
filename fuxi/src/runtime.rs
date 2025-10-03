use crate::types::{
    alias::{Price, Size},
    base::{Codes, Direction, Method, Side},
};
use anyhow::Result;

pub trait Runtime {
    fn run(&self) -> Result<()>;
    fn place_order(
        &self,
        code: Codes,
        method: Method,
        direction: Direction,
        side: Side,
        size: Size,
        price: Price,
    ) -> Result<String>;
    fn cancel_order(&self, code: Codes, id: &str) -> Result<()>;
}
