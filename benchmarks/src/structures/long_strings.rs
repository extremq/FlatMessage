use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::s;

#[derive(Clone, Serialize, Deserialize, FlatMessage, get_size_derive::GetSize)]
#[flat_message_options(store_name = false)]
pub struct LongStringStructure {
    string_one: String,
    string_two: String,
    string_three: String,
    string_four: String,
    value_one: u32,
    value_two: u64,
}

pub fn generate(count: usize) -> LongStringStructure {
    LongStringStructure {
        string_one: s("Hello, World".repeat(count)),
        string_two: s("How are you doing ?".repeat(count)),
        string_three: s("Testing".repeat(count)),
        string_four: s("X".repeat(count)),
        value_one: 1000000,
        value_two: 1000000000,
    }
}
