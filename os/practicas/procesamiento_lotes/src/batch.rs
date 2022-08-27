use crate::Process;

// batch class contains 3 processes, an id's for current process, time on execution
struct Batch {
    processes: [Process; 3],
    current_process: usize,
    time_on_execution: u8,
}

impl Batch {
    // constructor for batch class receives array of processes
    fn new(processes: [Process; 3]) -> Batch {
        Batch {
            processes,
            current_process: 0,
            time_on_execution: 0,
        }
    }

    // constructor for batch class receivers 3 processes
    fn new_with_processes(process1: Process, process2: Process, process3: Process) -> Batch {
        Batch {
            processes: [process1, process2, process3],
            current_process: 0,
            time_on_execution: 0,
        }
    }

    // method to execute batch
    fn execute(&mut self) {
        // while there are processes to execute
        while self.current_process < 3 {
            // while current process is not finished
            while self.processes[self.current_process].remaining_time > 0 {
                // execute current process
                self.processes[self.current_process].execute();
                // update time on execution
                self.time_on_execution += 1;
            }
            // update current process
            self.current_process += 1;
        }
    }

    // method to execute one step of batch
    fn execute_step(&mut self) {
        // while current process is not finished
        while self.processes[self.current_process].remaining_time > 0 {
            // execute current process
            self.processes[self.current_process].execute();
            // update time on execution
            self.time_on_execution += 1;
        }
        // update current process
        self.current_process += 1;
    }

    // method to get current process
    fn get_current_process(&self) -> &Process {
        &self.processes[self.current_process]
    }
}
