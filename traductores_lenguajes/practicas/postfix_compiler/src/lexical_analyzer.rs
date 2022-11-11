use crate::symbols::Token;

// Lexical Analyzer to tokenize mathematical expressions
pub struct LexicalAnalyzer {
    input: String,
    begin_pointer: usize,
    ahead_pointer: usize,
}

impl LexicalAnalyzer {
    pub fn new(input: String) -> LexicalAnalyzer {
        LexicalAnalyzer {
            input,
            begin_pointer: 0,
            ahead_pointer: 0,
        }
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.begin_pointer = self.ahead_pointer;
        if self.ahead_pointer >= self.input.len() {
            return None;
        }

        // Check if the next character is a math symbol, number, or identifier
        let c = self.input.chars().nth(self.ahead_pointer).unwrap();
        self.ahead_pointer += 1;
        match c {
            // Math symbols
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            _ => {
                // Numbers
                if c.is_numeric() {
                    self.read_number();
                    return Some(Token::Number(self.input[self.begin_pointer..self.ahead_pointer].parse().unwrap()));
                }
                // Identifiers
                if c.is_alphabetic() {
                    self.read_identifier();
                    return Some(Token::Identifier(self.input[self.begin_pointer..self.ahead_pointer].to_string()));
                }
                panic!("Unexpected character: {}", c);
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ahead_pointer < self.input.len() {
            let c = self.input.chars().nth(self.ahead_pointer).unwrap();
            if !c.is_whitespace() {
                break;
            }
            self.ahead_pointer += 1;
        }
    }

    fn read_number(&mut self) {
        while self.ahead_pointer < self.input.len() {
            let c = self.input.chars().nth(self.ahead_pointer).unwrap();
            if !c.is_numeric() {
                break;
            }
            self.ahead_pointer += 1;
        }
    }

    fn read_identifier(&mut self) {
        while self.ahead_pointer < self.input.len() {
            let c = self.input.chars().nth(self.ahead_pointer).unwrap();
            if !c.is_alphabetic() {
                break;
            }
            self.ahead_pointer += 1;
        }
    }
}