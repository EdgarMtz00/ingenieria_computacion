use crate::Process;
use crate::batch::Batch;

// state stores all batches, an id for current batch and total time on execution
pub struct State {
    batches: Vec<Batch>,
    current_batch: usize,
    time_on_execution: u8,
}

impl State {
    // constructor takes vector of processes and creates a batch for each 3 processes
    pub(crate) fn new(processes: Vec<Process>) -> State {
        let batches = Batch::batches_from_processes(processes);
        State {
            batches,
            current_batch: 0,
            time_on_execution: 0,
        }
    }

    // get current batch
    pub(crate) fn get_current_batch(&self) -> &Batch {
        if self.current_batch >= self.batches.len() {
            return &self.batches[self.batches.len() - 1];
        }
        &self.batches[self.current_batch]
    }

    // get remaining batches
    pub(crate) fn get_remaining_batches(&self) -> usize {
        self.batches.len() - self.current_batch
    }

    //is finished
    pub(crate) fn is_finished(&self) -> bool {
        self.current_batch == self.batches.len()
    }

    // take one step of execution
    pub(crate) fn execute_step(&mut self) -> bool {
        if self.current_batch < self.batches.len() {
            self.batches[self.current_batch].execute_step();
            self.time_on_execution += 1;
            if self.batches[self.current_batch].is_finished() {
                self.current_batch += 1;
            }
            return true;
        }
        false
    }

    // get time on execution
    pub(crate) fn get_time_on_execution(&self) -> u8 {
        self.time_on_execution
    }
}