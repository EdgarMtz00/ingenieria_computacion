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
    RealType
}