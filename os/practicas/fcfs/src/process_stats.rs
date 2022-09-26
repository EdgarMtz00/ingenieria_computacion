const TIME_ON_BLOCK:u32 = 7;

#[derive(Debug, Clone)]
pub struct ProcessStats{
    expected_time: u32,
    executed_time: u32,
    waited_time: u32,
    blocked_time: u32,
    arrival_time: Option<u32>,
    start_time: Option<u32>,
    end_time: Option<u32>
}

impl ProcessStats {
    pub fn new(expected_time: u32) -> Self { 
        Self { 
            expected_time, 
            executed_time: 0, 
            waited_time: 0,
            blocked_time: 0,
            arrival_time: None, 
            start_time: None, 
            end_time: None 
        } 
    }

    pub fn get_executed_time(&self) -> u32 {
        self.executed_time
    }

    pub fn increment_executed_time(&mut self) {
        self.executed_time += 1;
    }

    pub fn get_expected_time(&self) -> u32 {
        self.expected_time
    }

    pub fn get_blocked_time(&self) -> u32 {
        self.blocked_time
    }

    pub fn increment_blocked_time(&mut self) -> bool {
        if self.blocked_time < TIME_ON_BLOCK {
            self.blocked_time += 1;
            true
        } else {
            self.blocked_time = 0;
            false
        }
    }

    pub fn get_arrival_time(&self) -> Option<u32> {
        self.arrival_time
    }

    pub fn set_arrival_time(&mut self, arrival_time: u32) {
        match self.arrival_time {
            Some(_) => panic!("You shouldn't reset the arrival time"),
            None => self.arrival_time = Some(arrival_time),
        }
    }

    pub fn get_start_time(&self) -> Option<u32> {
        self.start_time
    }

    pub fn set_start_time(&mut self, start_time: u32) {
        match self.start_time {
            Some(_) => (),
            None => self.start_time = Some(start_time),
        }
    }

    pub fn get_end_time(&self) -> Option<u32> {
        self.end_time
    }

    pub fn set_end_time(&mut self, end_time: u32) {
        match self.end_time {
            Some(_) => panic!("You shouldn't reset the end time"),
            None => self.end_time = Some(end_time),
        }
    }

    pub fn get_waited_time(&self) -> u32 {
        self.waited_time
    }

    pub fn increment_waited_time(&mut self) {
        self.waited_time += 1;
    }

    pub fn arrival_to_end_time(&self) -> Result<u32, &'static str>{
        let arrival = self.arrival_time.expect("Process hasn't arrived yet");
        let end = self.end_time.expect("Process hasn't ended yet");

        Ok(end - arrival)
    }

    pub fn arrival_to_start_time(&self) -> Result<u32, &'static str>{
        let arrival = self.arrival_time.expect("Process hasn't arrived yet");
        let start = self.start_time.expect("Process hasn't started yet");

       Ok(start -  arrival)
    }

    pub fn get_remaining_time(&self) -> u32 {
        self.expected_time - self.executed_time
    }
}

