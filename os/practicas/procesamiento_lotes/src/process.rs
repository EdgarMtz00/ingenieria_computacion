use std::fmt::Display;
use crate::operation::Operation;

#[derive(Clone, PartialEq)]
pub struct Process {
    pub id: u8,
    expected_time: u8,
    pub remaining_time: u8,
    pub(crate) elapsed_time: u8,
    operation: Operation,
}

impl Process {
    pub fn new(id: u8, expected_time: u8, operation: Operation) -> Self {
        Process {
            id,
            expected_time,
            remaining_time: expected_time,
            elapsed_time: 0,
            operation,
        }
    }

    // method to execute process
    pub fn execute(&mut self) {
        if self.remaining_time > 0 {
            self.elapsed_time += 1;
            self.remaining_time -= 1;
        }
    }

    // method to get name and remaining time
    pub fn get_name_and_remaining_time(&self) -> String {
        format!("#{} - {}", self.id, self.remaining_time)
    }

    // method to get name, operation and it's result
    pub fn get_name_operation_result(&self) -> String {
        match self.operation.result {
            Some(result) => format!("#{} | {} = {}", self.id, self.operation, result),
            None => format!("#{} | {} = Error", self.id, self.operation),
        }
    }

    // method to end without a result
    pub fn end_without_result(&mut self) {
        self.remaining_time = 0;
        self.operation.result = None;
    }

}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Proceso #{}\n Tiempo esperado: {}\n Tiempo restante: {}\n Tiempo transcurrido: {}\n Operacion: {}",
               self.id, self.expected_time, self.remaining_time, self.elapsed_time, self.operation)
    }
}

