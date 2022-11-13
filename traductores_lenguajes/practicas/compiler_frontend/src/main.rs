mod symbols;
mod lexical_analyzer;

fn main() {
    let input =
        "begin \
            entero x;\
            x:=10.9;\
            while (x>=0) \
                x:=x-1 \
            endwhile
        end";

    let mut lexer = lexical_analyzer::LexicalAnalyzer::new(input.to_string());
    let tokens = lexer.tokens();
    for token in &tokens {
        println!("{:?}", token);
    }
}
