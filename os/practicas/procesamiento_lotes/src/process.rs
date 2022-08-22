use std::str::FromStr;
use crate::operation::Operation;

pub struct Process {
    id: u8,
    name: String,
    expected_time: u8,
    remaining_time: u8,
    elapsed_time: u8,
    operation: Operation,
}

impl Process {
    pub fn new(id: u8, name: String, expected_time: u8, operation_str: &str) -> Self {
        Process{
            id,
            name,
            expected_time,
            remaining_time: expected_time,
            elapsed_time: 0,
            operation: Operation::from_str(operation_str).unwrap()
        }
    }
}