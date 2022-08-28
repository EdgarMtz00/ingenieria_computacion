use crate::Process;

// batch class contains 3 processes, an id's for current process, time on execution
pub struct Batch {
    processes: [Option<Process>; 3],
    current_process: usize,
    time_on_execution: u8,
}

impl Batch {
    // create a vector of batches with 3 processes each from a vector of processes
    pub fn batches_from_processes(mut processes: Vec<Process>) -> Vec<Batch> {
        let mut batches = Vec::new();
        for _ in 0..processes.len() / 3 + 1 {
            if processes.len() >= 3 {
                batches.push(Batch::new_with_processes(Some(processes.remove(0)), Some(processes.remove(0)), Some(processes.remove(0))));
            } else  if processes.len() == 2 {
                batches.push(Batch::new_with_processes(Some(processes.remove(0)), Some(processes.remove(0)), None));
            } else if processes.len() == 1 {
                batches.push(Batch::new_with_processes(Some(processes.remove(0)), None, None));
            }

        }
        batches
    }

    // constructor from 3 processes
    pub fn new_with_processes(process1: Option<Process>, process2: Option<Process>, process3: Option<Process>) -> Batch {
        Batch {
            processes: [process1, process2, process3],
            current_process: 0,
            time_on_execution: 0,
        }
    }

    // method to execute one step of batch
    pub fn execute_step(&mut self) {
        let current_process = self.processes[self.current_process].as_mut().unwrap();

        // execute current process
        current_process.execute();
        // update time on execution
        self.time_on_execution += 1;

        // update current process
        if current_process.remaining_time == 0 {
            self.current_process += 1;
            if self.current_process != 3 && self.processes[self.current_process].is_none() {
                self.current_process = 3;
            }
        }
    }

    // method to get current process
    pub fn get_current_process(&self) -> Option<&Process> {
        if self.current_process >= self.processes.len() {
            return None;
        }
        self.processes[self.current_process].as_ref()
    }

    // method to check if batch is finished
    pub fn is_finished(&self) -> bool {
        self.current_process == 3
    }

    // method to get pending processes
    pub fn get_pending_processes(&self) -> Vec<&Process> {
        let mut pending_processes = Vec::new();
        for process in &self.processes {
            if process.is_some() {
                let p = process.as_ref().unwrap();
                if p.remaining_time > 0 {
                    pending_processes.push(p);
                }
            }
        }
        pending_processes
    }

    // get finished processes
    pub fn get_finished_processes(&self) -> Vec<&Process> {
        let mut finished_processes = Vec::new();
        for process in &self.processes {
            if process.is_some() {
                let p = process.as_ref().unwrap();
                if p.remaining_time == 0 {
                    finished_processes.push(p);
                }
            }
        }
        finished_processes
    }
}