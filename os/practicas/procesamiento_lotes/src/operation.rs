use std::str::FromStr;

#[derive(PartialEq)]
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
            Operator::Add => {first_operand + second_operand}
            Operator::Subtract => {first_operand - second_operand}
            Operator::Multiply => {first_operand * second_operand}
            Operator::Divide => {first_operand / second_operand}
        }
    }
}

pub struct Operation {
    first_operand: isize,
    second_operand: isize,
    operator: Operator,
    pub result: Option<isize>,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(" ").collect();

        if tokens.len() != 3 {
            return Err(String::from("Formato de operacion invalido"));
        }

        let operator: Operator = tokens[1].parse().unwrap();
        let first_operand: isize = tokens[0].parse().unwrap();
        let second_operand: isize = tokens[2].parse().unwrap();
        if operator == Operator::Divide && second_operand == 0 {
            return Err(String::from("No se puede dividir entre 0"));
        }
        let result = Some(operator.do_operation(first_operand, second_operand));

        return Ok(Operation{
            first_operand,
            second_operand,
            operator,
            result
        })
    }
}

impl Operation {
    pub fn new_empty() -> Self {
        Operation{
            first_operand: 0,
            second_operand: 0,
            operator: Operator::Add,
            result: None
        }
    }
}
