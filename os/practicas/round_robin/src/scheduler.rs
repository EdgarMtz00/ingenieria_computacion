use std::collections::VecDeque;
use crate::process::Process;

const MAX_PROCESSES: usize = 3;
const MAX_QUANTUM: u32 = 5;

pub struct Scheduler {
    max_id: u8,
    processes: VecDeque<Process>,
    waiting_queue: VecDeque<Process>,
    executing: Option<Process>,
    blocked: Vec<Process>,
    finished: Vec<Process>,
    time: u32,
    quantum: u32,
}

impl Scheduler {
   pub fn new(mut processes: Vec<Process>) -> Self {
       let mut waiting_queue = VecDeque::new();
       let mut left_processes: VecDeque<Process> = VecDeque::new();
       let max_id = processes.len() as u8;

       if processes.len() > MAX_PROCESSES {
           left_processes = VecDeque::from(processes.split_off(MAX_PROCESSES));
       }

       for mut process in processes {
           process.load(0);
           waiting_queue.push_back(process);
       }

       let mut executing = waiting_queue.pop_front().unwrap_or_else(|| panic!("No processes to execute"));
       executing.start(0);

       Self {
           max_id,
           processes: left_processes,
           waiting_queue,
           executing: Some(executing),
           blocked: Vec::new(),
           finished: Vec::new(),
           time: 0,
           quantum: MAX_QUANTUM,
       }
   }

    pub fn add_new(&mut self) {
        let mut new_process = Process::new_random(self.max_id);
        self.max_id += 1;
        let mut active_processes = self.waiting_queue.len() + self.blocked.len();
        if self.executing.is_some() {
            active_processes += 1;
        }
        if active_processes < MAX_PROCESSES {
            new_process.load(self.time);
            self.waiting_queue.push_back(new_process);

        } else {
            self.processes.push_back(new_process);
        }
    }

    pub fn execute(&mut self) {
        if self.quantum == 0 {
            self.quantum = MAX_QUANTUM;
            if let Some(executing) = self.executing.take() {
                self.waiting_queue.push_back(executing);
                self.start_new_one();
            }
        }
        self.time += 1;
        self.execute_executing();
        self.execute_waiting_queue();
        self.execute_blocked();
        self.quantum -= 1;
    }

    fn execute_waiting_queue(&mut self) {
        if self.waiting_queue.is_empty() {
            return;
        }

        for process in &mut self.waiting_queue {
            process.wait();
        }
    }

    fn execute_executing(&mut self) -> (){
        if self.executing.is_none() {
            self.start_new_one();
            return;
        }

        let executing = self.executing.as_mut().unwrap();
        executing.execute();
        if executing.is_finished() {
            self.end_executing();
        }
    }

    fn end_executing(&mut self) {
        let executing = self.executing.take().unwrap();
        self.finished.push(executing.clone());

        self.start_new_one();

        let mut total_processes = self.waiting_queue.len() + self.blocked.len();
        if self.executing.is_some() {
            total_processes += 1;
        }
        if total_processes < MAX_PROCESSES && !self.processes.is_empty() {
            let mut new_process = self.processes.pop_front().unwrap();
            new_process.load(self.time);
            self.waiting_queue.push_back(new_process);
        }
    }

    fn start_new_one(&mut self) {
        if self.executing.is_some() {
            return;
        }
        self.quantum = MAX_QUANTUM;
        self.executing = self.waiting_queue.pop_front();
        if self.executing.is_some() {
            let executing = self.executing.as_mut().unwrap();
            executing.start(self.time);
        }
    }

    fn execute_blocked(&mut self) {
        if self.blocked.is_empty() {
            return;
        }

        let mut blocked_ended = false;

        for process in self.blocked.iter_mut() {
            if !process.block() {
                self.waiting_queue.push_back(process.clone());
                blocked_ended = true;
            }
        }
        if blocked_ended {
            self.blocked.remove(0);
        }
    }

    pub fn error(&mut self) {
        if self.executing.is_none() {
            return;
        }

        let executing = self.executing.as_mut().unwrap();
        executing.end_without_result(self.time);
        self.end_executing();
    }

    pub fn block(&mut self) {
        if self.executing.is_none() {
            return;
        }

        let executing = self.executing.as_mut().unwrap();
        self.blocked.push(executing.clone());
        self.executing = None;
        self.start_new_one();
    }

    pub fn get_waiting_processes(&self) -> &VecDeque<Process> {
        &self.waiting_queue
    }

    pub fn get_executing_process(&self) -> &Option<Process> {
        &self.executing
    }

    pub fn get_blocked_processes(&self) -> &Vec<Process> {
        &self.blocked
    }

    pub fn get_finished_processes(&self) -> &Vec<Process> {
        &self.finished
    }

    pub fn get_processes(&self) -> &VecDeque<Process> {
        &self.processes
    }

    pub fn get_time(&self) -> &u32 {
        &self.time
    }

    pub fn get_bcp(&self) -> Vec<Vec<String>> {
        let mut bcp = Vec::new();

        for process in &self.finished {
            bcp.push(process.bcp_array());
        }
        if self.executing.is_some() {
            bcp.push(self.executing.as_ref().unwrap().bcp_array());
        }
        for process in &self.blocked {
            bcp.push(process.bcp_array());
        }
        for process in &self.waiting_queue {
            bcp.push(process.bcp_array());
        }
        for process in &self.processes {
            bcp.push(process.bcp_array());
        }
        bcp
    }

    pub fn get_quantum(&self) -> u32 {
        &self.quantum + 1
    }
}