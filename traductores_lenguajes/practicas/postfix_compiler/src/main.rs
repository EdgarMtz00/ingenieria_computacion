mod lexical_analyzer;
mod syntax_analyzer;
mod symbols;

fn main() {
    let input = "15 + 200 * 3 / (a / 5 - b) + 6";

    let mut lexer = lexical_analyzer::LexicalAnalyzer::new(input.to_string());
    let tokens = lexer.tokens();
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = syntax_analyzer::Parser::new(tokens);
    println!("Input: {}\nOutput: {}", input, parser.expression());
}
