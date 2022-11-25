use crate::symbols::Operation;

#[derive(Debug)]
pub struct Quadruple {
    pub result: String,
    pub op: Operation,
    pub arg1: Option<String>,
    pub arg2: Option<String>,
}


pub fn condition_quadruple(comparison: String, mut orders: Vec<Quadruple>, is_else: bool ) -> Vec<Quadruple> {
    let mut quadruples = Vec::new();
    let condition_start = Quadruple {
        op: if is_else { Operation::GotoIf } else { Operation::GotoIfNot },
        arg1: Some((orders.len() + 1).to_string()),
        arg2: Some(comparison),
        result: "".to_string(),
    };
    quadruples.push(condition_start);
    quadruples.append(&mut orders);
    quadruples
}

pub fn while_loop_quadruple(comparison: String, mut orders: Vec<Quadruple>) -> Vec<Quadruple> {
    let mut quadruples = Vec::new();
    let orders_len = orders.len() as isize;
    let condition_start = Quadruple {
        op: Operation::GotoIfNot,
        arg1: Some((orders_len + 2).to_string()),
        arg2: Some(comparison),
        result: "".to_string(),
    };
    quadruples.push(condition_start);
    quadruples.append(&mut orders);

    let condition_end = Quadruple {
        op: Operation::Goto,
        arg1: Some((-orders_len - 2).to_string()),
        arg2: None,
        result: "".to_string(),
    };
    quadruples.push(condition_end);
    quadruples
}