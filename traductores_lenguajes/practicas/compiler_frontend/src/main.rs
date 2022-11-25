mod symbols;
mod lexical_analyzer;
mod syntax_analyzer;
mod three_address_code;

fn main() {
    let input =
        "begin \n\
            entero x;\n\
            entero y;\n\

            x:=10.9;\n\
            y:=x+2;\n\

            if (x>2)\n\
                y:=y+1;\n\
            else\n\
                y:=y-1;\n\
            end\n\

            while (x>=0) \n\
                x:=x-1; \n\
            endwhile\n\
        end\n";

    print!("Input:\n\n{}\n\n", input);

    let mut lexer = lexical_analyzer::LexicalAnalyzer::new(input.to_string());
    let tokens = lexer.tokens();
    for token in &tokens {
        println!("{:?}", token);
    }
    println!("\n\n");

    let mut parser = syntax_analyzer::Parser::new(tokens);
    let quadruples = parser.program();
    for quadruple in &quadruples {
        println!("{:?}", quadruple);
    }
    println!("\n\n");
}
