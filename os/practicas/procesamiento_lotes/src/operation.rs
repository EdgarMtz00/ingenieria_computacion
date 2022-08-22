use std::str::FromStr;

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
    fn do_operation(&self, first_operand: usize, second_operand: usize) -> usize{
        match self {
            Operator::Add => {first_operand + second_operand}
            Operator::Subtract => {first_operand - second_operand}
            Operator::Multiply => {first_operand * second_operand}
            Operator::Divide => {first_operand / second_operand}
        }
    }
}

pub struct Operation {
    first_operand: usize,
    second_operand: usize,
    operator: Operator,
    pub result: Option<usize>,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(" ").collect();

        if tokens.len() != 3 {
            return Err(());
        }

        let operator: Operator = tokens[1].parse().unwrap();
        let first_operand: usize = tokens[0].parse().unwrap();
        let second_operand: usize = tokens[2].parse().unwrap();
        let result = Some(operator.do_operation(first_operand, second_operand));

        return Ok(Operation{
            first_operand,
            second_operand,
            operator,
            result
        })
    }
}
