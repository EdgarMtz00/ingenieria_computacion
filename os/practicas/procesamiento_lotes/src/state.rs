use rand::Rng;
use crate::{Operation, Process};
use crate::batch::Batch;
use crate::operation::Operator;

// state stores all batches, an id for current batch and total time on execution
pub struct State {
    batches: Vec<Batch>,
    current_batch: usize,
    time_on_execution: u8,
}

impl State {
    pub fn new(num_processes: usize) -> State {
        let mut processes = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..num_processes {
            processes.push(
                Process::new(
                    i as u8,
                    rng.gen_range(5..15),
                    Operation::new(
                        rng.gen_range(0..15) as usize,
                        rng.gen_range(1..15) as usize,
                        rng.gen::<Operator>()
                    )
                )
            );
        }
        State {
            batches: Batch::batches_from_processes(processes),
            current_batch: 0,
            time_on_execution: 0,
        }
    }

    // get finished processes
    pub(crate) fn get_finished_processes(&self) -> Vec<&Process> {
        let mut finished_processes = Vec::new();
        for batch in &self.batches {
            for process in batch.processes.iter().flatten() {
                if process.remaining_time == 0 {
                    finished_processes.push(process);
                }
            }
        }
        finished_processes
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

    pub fn end_process_without_result(&mut self) {
        self.batches[self.current_batch].end_process_without_result();
    }

    pub fn interrupt(&mut self) {
        self.batches[self.current_batch].interrupt();
    }
}