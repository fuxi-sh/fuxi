use crate::types::{
    alias::{Price, Size},
    base::{Codes, Direction, Method, Mode, Side},
};
use anyhow::Result;

pub trait Runtime {
    fn mode(&self) -> Mode;
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
