use crate::get_size_min::GetSize;
use flat_message::*;
use serde::{Deserialize, Serialize};

use crate::s;

#[derive(Clone, Serialize, Deserialize, FlatMessage, get_size_derive::GetSize, bincode::Encode, bincode::Decode)]
pub struct ProcessCreated {
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: u32,
    unique_id: u32,
}

pub fn generate() -> ProcessCreated {
    ProcessCreated {
        name: s(String::from("C:\\Windows\\System32\\example.exe")),
        pid: 1234,
        parent_pid: 1,
        parent: s(String::from("C:\\Windows\\System32\\explorer.exe")),
        user: s(String::from("Administrator")),
        command_line: s(String::from("-help -verbose -debug -output C:\\output.txt")),
        timestamp: 0xFEFEFEFE as u32,
        unique_id: 0xABABABAB as u32,
    }
}
