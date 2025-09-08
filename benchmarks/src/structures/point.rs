use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, FlatMessage,  get_size_derive::GetSize, bincode::Encode, bincode::Decode)]
#[flat_message_options(store_name = false)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn generate() -> Point {
    Point { x: -10, y: 100 }
}
