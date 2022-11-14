// Token enum for math expressions with numbers, and identifiers
#[derive(Debug, PartialEq)]
pub enum Token {
    // Math symbols
    Plus,
    Minus,
    Multiply,
    Divide,
    // Parenthesis
    LeftParen,
    RightParen,
    // Numbers
    Number(f64),
    // Identifiers
    Identifier(String),
}