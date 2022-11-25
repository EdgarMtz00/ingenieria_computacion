use std::collections::HashMap;

// Token enum for math expressions with numbers, and identifiers
#[derive(Debug, PartialEq)]
pub enum Token {
    // Math symbols
    Plus,
    Minus,
    Multiply,
    Divide,
    // Parenthesis
    LeftParenthesis,
    RightParenthesis,
    // Numbers
    Integer(u64),
    Real(f64),
    // Identifiers
    Identifier(String),
    Begin,
    End,
    If,
    Else,
    While,
    EndWhile,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    NotEqual,
    Assign,
    Semicolon,
    IntegerType,
    RealType,
    Comma
}

#[derive(Debug)]
pub enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    NotEqual,
    Goto,
    GotoIf,
    GotoIfNot,
}

pub struct SymbolData {
    pub data_type: String,
}

pub struct SymbolTable {
    pub symbols: HashMap<String, SymbolData>,
    temp_counter: u64,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
            temp_counter: 0,
        }
    }

    pub fn add_symbol(&mut self, name: String, data_type: String) {
        let symbol = SymbolData {
            data_type,
        };
        self.symbols.insert(name, symbol);
    }

    pub fn new_temp(&mut self, data_type: String) -> String {
        let temp_name = format!("temp_{}", self.temp_counter);
        self.temp_counter += 1;
        self.add_symbol(temp_name.clone(), data_type);
        temp_name
    }


}