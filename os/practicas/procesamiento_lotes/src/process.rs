use crate::operation::Operation;

pub struct Process {
    pub(crate) id: u8,
    name: String,
    expected_time: u8,
    pub(crate) remaining_time: u8,
    elapsed_time: u8,
    operation: Operation,
}

impl Process {
    pub fn new(id: u8, name: String, expected_time: u8, operation: Operation) -> Self {
        Process {
            id,
            name,
            expected_time,
            remaining_time: expected_time,
            elapsed_time: 0,
            operation,
        }
    }

    // method to execute process
    pub fn execute(&mut self) {
        self.elapsed_time += 1;
        self.remaining_time -= 1;
    }

    // method to represent as string
    pub fn to_string(&self) -> String {
        format!("Proceso #{} - {}\n Tiempo esperado: {}\n Tiempo restante: {}\n Tiempo transcurrido: {}\n Operacion: {}",
                self.id, self.name, self.expected_time, self.remaining_time, self.elapsed_time, self.operation.to_string())
    }

    // method to get name and remaining time
    pub fn get_name_and_remaining_time(&self) -> String {
        format!("{} - {}", self.name, self.remaining_time)
    }

    // method to get name, operation and it's result
    pub fn get_name_operation_result(&self) -> String {
        format!("#{} {} - {} = {}", self.id, self.name, self.operation.to_string(), self.operation.result.unwrap())
    }
}