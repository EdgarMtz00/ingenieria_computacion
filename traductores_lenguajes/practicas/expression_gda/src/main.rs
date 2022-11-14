mod lexical_analyzer;
mod syntax_analyzer;
mod symbols;
mod gda;

fn main() {
    let input = "(b - c) + d + (b - c)";

    let mut lexer = lexical_analyzer::LexicalAnalyzer::new(input.to_string());
    let tokens = lexer.tokens();

    let mut parser = syntax_analyzer::Parser::new(tokens);
    let tree = parser.expression();
    tree.print(0);
}
