pub struct ProcessStats{
    expected_time: u32,
    executed_time: u32,
    waited_time: u32,
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
            arrival_time: None, 
            start_time: None, 
            end_time: None 
        } 
    }

    pub fn executed_time(&self) -> u32 {
        self.executed_time
    }

    pub fn increment_executed_time(&mut self) {
        self.executed_time += 1;
    }

    pub fn expected_time(&self) -> u32 {
        self.expected_time
    }

    pub fn arrival_time(&self) -> Option<u32> {
        self.arrival_time
    }

    pub fn set_arrival_time(&mut self, arrival_time: u32) {
        match self.arrival_time {
            Some(_) => panic!("You shouldn't reset the arrival time"),
            None => self.arrival_time = Some(arrival_time),
        }
    }

    pub fn start_time(&self) -> Option<u32> {
        self.start_time
    }

    pub fn set_start_time(&mut self, start_time: Option<u32>) {
        match self.start_time {
            Some(_) => panic!("You shouldn't reset the start time"),
            None => self.start_time = start_time,
        }
    }

    pub fn end_time(&self) -> Option<u32> {
        self.end_time
    }

    pub fn set_end_time(&mut self, end_time: Option<u32>) {
        match self.end_time {
            Some(_) => panic!("You shouldn't reset the end time"),
            None => self.end_time = end_time,
        }
    }

    pub fn waited_time(&self) -> u32 {
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
}

