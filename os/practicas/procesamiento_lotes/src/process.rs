use std::str::FromStr;
use crate::operation::Operation;

pub struct Process {
    id: u8,
    name: String,
    expected_time: u8,
    pub(crate) remaining_time: u8,
    elapsed_time: u8,
    operation: Operation,
}

impl Process {
    pub fn new(id: u8, name: String, expected_time: u8, operation_str: &str) -> Self {
        Process {
            id,
            name,
            expected_time,
            remaining_time: expected_time,
            elapsed_time: 0,
            operation: Operation::from_str(operation_str).unwrap(),
        }
    }

    // method to execute process
    pub fn execute(&mut self) {
        self.elapsed_time += 1;
        self.remaining_time -= 1;
    }

    // method to represent as string
    pub fn to_string(&self) -> String {
        format!("Proceso #{} - {}\n Tiempo esperado: {}\n Tiempo restante: {}\n Tiempo transcurrido: {}",
                self.id, self.name, self.expected_time, self.remaining_time, self.elapsed_time)
    }
}