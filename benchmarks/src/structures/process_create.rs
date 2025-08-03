use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::s;

#[derive(Clone, Serialize, Deserialize, FlatMessage, get_size_derive::GetSize)]
pub struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,
}

pub fn generate_flat() -> ProcessCreated {
    ProcessCreated {
        name: s(String::from("C:\\Windows\\System32\\example.exe")),
        pid: 1234,
        parent_pid: 1,
        parent: s(String::from("C:\\Windows\\System32\\explorer.exe")),
        user: s(String::from("Administrator")),
        command_line: s(String::from("-help -verbose -debug -output C:\\output.txt")),
        timestamp: flat_message::Timestamp::with_value(0xFEFEFEFE),
        unique_id: flat_message::UniqueID::with_value(0xABABABAB),
    }
}
