use std::str::FromStr;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use text_io::read;

#[derive(PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(())
        }
    }
}

impl Operator {
    fn do_operation(&self, first_operand: isize, second_operand: isize) -> isize{
        match self {
            Operator::Add => { first_operand + second_operand }
            Operator::Subtract => { first_operand - second_operand }
            Operator::Multiply => { first_operand * second_operand }
            Operator::Divide => { first_operand / second_operand }
        }
    }
}

// implement display trait for Operator
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
        }
    }
}

impl Distribution<Operator> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Operator {
        match rng.gen_range(0..4) {
            0 => Operator::Add,
            1 => Operator::Subtract,
            2 => Operator::Multiply,
            3 => Operator::Divide,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Operation {
    first_operand: usize,
    second_operand: usize,
    operator: Operator,
    result: Option<isize>,
}

impl Operation {
    pub fn new(first_operand: usize, second_operand: usize, operator: Operator) -> Self {
        let result = None;
        Operation{
            first_operand,
            second_operand,
            operator,
            result
        }
    }

    pub fn set_result(&mut self){
        self.result = Some(self.operator.do_operation(self.first_operand as isize, self.second_operand as isize));
    }

    pub fn get_result(&self) -> String {
        match self.result {
            None => "Error".to_string(),
            Some(result) => result.to_string(),
        }
    }

    pub fn error(&mut self) {
        self.result = None;
    }
}

// implement display trait for operation
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.result {
            None => write!(f, "{} {} {} = N/A", self.first_operand, self.operator, self.second_operand),
            Some(result) => write!(f, "{} {} {} = {}", self.first_operand, self.operator, self.second_operand, result),
        }
    }
}
