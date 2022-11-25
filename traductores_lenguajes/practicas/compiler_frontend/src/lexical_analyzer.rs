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
            '(' => Some(Token::LeftParenthesis),
            ')' => Some(Token::RightParenthesis),
            ';' => Some(Token::Semicolon),
            '=' => Some(Token::Equal),
            ',' => Some(Token::Comma),
            ':' => {
                let next_c = self.input.chars().nth(self.ahead_pointer).unwrap();
                if next_c == '='{
                    self.ahead_pointer += 1;
                    return Some(Token::Assign);
                }
                panic!("Unexpected character: {}", c);
            },
            '<' => {
                let next_c = self.input.chars().nth(self.ahead_pointer).unwrap();
                return match next_c {
                    '=' => {
                        self.ahead_pointer += 1;
                        Some(Token::LessThanEqual)
                    },
                    '>' => {
                        self.ahead_pointer += 1;
                        Some(Token::NotEqual)
                    },
                    _ => Some(Token::LessThan)
                }
            },
            '>' => {
                let next_c = self.input.chars().nth(self.ahead_pointer).unwrap();
                if next_c == '=' {
                    self.ahead_pointer += 1;
                    return Some(Token::GreaterThanEqual);
                }
                Some(Token::GreaterThan)
            }
            _ => {
                self.ahead_pointer -= 1;
                if let Some(key_word) = self.read_key_words() {
                    return Some(key_word)
                }
                // Numbers
                if c.is_numeric() {
                    let is_integer = self.read_number();
                    return if is_integer {
                        Some(Token::Integer(self.input[self.begin_pointer..self.ahead_pointer].parse().unwrap()))
                    } else {
                        Some(Token::Real(self.input[self.begin_pointer..self.ahead_pointer].parse().unwrap()))
                    }
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

    fn read_key_words(&mut self) -> Option<Token> {
        let word: String = self.input.chars().skip(self.ahead_pointer).take_while(|c| !c.is_whitespace()).collect();
        let word = word.as_str();
        let result = match word {
            "begin" => Some(Token::Begin),
            "end" => Some(Token::End),
            "entero" => Some(Token::IntegerType),
            "real" => Some(Token::RealType),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "endwhile" => Some(Token::EndWhile),
            _ => None
        };
        if result.is_some() {
            self.ahead_pointer += word.len();
        }
        result
    }

    fn read_number(&mut self) -> bool{
        let mut is_integer = true;
        while self.ahead_pointer < self.input.len() {
            let c = self.input.chars().nth(self.ahead_pointer).unwrap();
            if c == '.' {
                is_integer = false;
            } else if !c.is_numeric() {
                break;
            }
            self.ahead_pointer += 1;
        }
        return is_integer
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