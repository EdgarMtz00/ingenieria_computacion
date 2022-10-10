use rand::Rng;
use crate::operation::{Operation, Operator};
use crate::process_stats::ProcessStats;

#[derive(Clone)]
pub struct Process {
    id: u8,
    operation: Operation,
    stats: ProcessStats,
    finished: bool,
}

impl Process {
    pub fn new(id: u8, operation: Operation, expected_time: u32) -> Self {
        Process {
            id,
            operation,
            stats: ProcessStats::new(expected_time),
            finished: false,
        }
    }

    pub fn new_random(id: u8) -> Self {
        let mut rng = rand::thread_rng();
        Process::new(id, Operation::new(
            rng.gen_range(0..15) as usize,
            rng.gen_range(1..15) as usize,
            rng.gen::<Operator>()
        ), rng.gen_range(5..10))
    }

    pub fn random_vector(num_processes: usize) -> Vec<Self> {
        let mut processes = Vec::new();
        for i in 0..num_processes {
            processes.push(Process::new_random(i as u8));
        }
        processes
    }

    pub fn start(&mut self, time: u32) {
        self.stats.set_start_time(time);
    }

    pub fn load(&mut self, time: u32) {
        self.stats.set_arrival_time(time);
    }

    pub fn execute(&mut self) {
        self.stats.increment_executed_time();
        if self.stats.get_remaining_time() == 0 {
            self.stats.set_end_time(self.stats.get_start_time().unwrap() + self.stats.get_executed_time() + self.stats.get_waited_time());
            self.operation.set_result();
            self.finished = true;
        }
    }

    pub fn block(&mut self) -> bool {
        self.stats.increment_waited_time();
        self.stats.increment_blocked_time()
    }

    pub fn wait(&mut self) {
        self.stats.increment_waited_time();
    }

    pub fn end_without_result(&mut self, time: u32) {
        self.operation.error();
        self.stats.set_end_time(time);
        self.finished = true;
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn get_stats(&self) -> &ProcessStats {
        &self.stats
    }

    pub fn get_operation(&self) -> &Operation {
        &self.operation
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_status(&self) -> String {
        match self.finished {
            true => "Terminado".to_string(),
            false => {
                if self.stats.get_arrival_time().is_some() {
                    "Listo".to_string()
                } else {
                    "Nuevo".to_string()
                }
            },
        }
    }

    pub fn bcp_array(&self) -> Vec<String> {
        let mut bcp: Vec<String> = Vec::new();
        bcp.push(self.id.to_string().to_owned());
        bcp.push(self.get_status());
        bcp.push(self.operation.to_string().to_owned());

        match self.stats.get_arrival_time() {
            None =>  bcp.push("N/A".to_string()),
            Some(time) => bcp.push(time.to_string()),
        }

        match self.stats.get_end_time() {
            None => bcp.push("N/A".to_string()),
            Some(time) => bcp.push(time.to_string()),
        }

        match self.stats.arrival_to_end_time() {
            Ok(time) => bcp.push(time.to_string()),
            Err(_) => bcp.push("N/A".to_string()),
        }

        bcp.push(self.stats.get_waited_time().to_string());
        bcp.push(self.stats.get_executed_time().to_string());
        bcp.push(self.stats.get_remaining_time().to_string());

        match self.stats.arrival_to_start_time() {
            Err(_) => bcp.push("N/A".to_string()),
            Ok(time) => bcp.push(time.to_string()),
        }

        bcp
    }
}