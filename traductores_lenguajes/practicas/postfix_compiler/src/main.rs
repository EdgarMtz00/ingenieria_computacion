mod lexical_analyzer;
mod syntax_analyzer;
mod symbols;

fn solve_postfix(postfix: String) -> i32 {
    let mut stack = Vec::new();
    for token in postfix.split_whitespace() {
        match token {
            "+" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left + right);
            }
            "-" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left - right);
            }
            "*" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left * right);
            }
            "/" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left / right);
            }
            _ => stack.push(token.parse::<i32>().unwrap()),
        }
    }
    stack.pop().unwrap()
}

fn main() {
    let input = "4 + 6 / ( 1 - 3 )";

    let mut lexer = lexical_analyzer::LexicalAnalyzer::new(input.to_string());
    let tokens = lexer.tokens();
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = syntax_analyzer::Parser::new(tokens);
    let postfix = parser.expression();
    println!("Input: {}\nOutput: {}", input, postfix);

    if postfix.chars().all(|c| c.is_numeric() || c.is_whitespace() || c == '+' || c == '-' || c == '*' || c == '/') {
        println!("Result: {}", solve_postfix(postfix));
    }
}
