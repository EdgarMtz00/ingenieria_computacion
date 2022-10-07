use crate::file_system::{initialize_files, read_temp_file, write_output_file, write_temp_file};

fn expr() -> f32 {
    let term_result = term();
    expr_tail( term_result)
}

fn expr_tail(inherited: f32) -> f32 {
    match peek() {
        '+' => {
            consume('+');
            expr_tail( inherited + term())
        }
        '-' => {
            consume('-');
            expr_tail( inherited - term())
        }
        _ => inherited
    }
}

fn term() -> f32 {
    term_tail(factor())
}

fn term_tail( inherited: f32) -> f32 {
    match peek() {
        '*' => {
            consume('*');
            term_tail( inherited * factor())
        }
        '/' => {
            consume('/');
            term_tail( inherited / factor())
        }
        _ => inherited
    }
}

fn factor() -> f32 {
    match peek() {
        '(' => {
            consume('(');
            let result = expr();
            consume(')');
            result
        }
        _ => digit()
    }
}

fn digit() -> f32 {
    let peek = peek();
    match peek {
        '0' => {
            consume('0');
            0.0
        },
        '1' => {
            consume('1');
            1.0
        },
        '2' => {
            consume('2');
            2.0
        },
        '3' => {
            consume('3');
            3.0
        },
        '4' => {
            consume('4');
            4.0
        },
        '5' => {
            consume('5');
            5.0
        },
        '6' => {
            consume('6');
            6.0
        },
        '7' => {
            consume('7');
            7.0
        },
        '8' => {
            consume('8');
            8.0
        },
        '9' => {
            consume('9');
            9.0
        },
        _ => panic!("Unexpected character: {}", peek)
    }
}

fn consume(expected: char) {
    let contents = read_temp_file();

    let chars = contents.chars();
    let mut whitespaces = 0;
    let consumed = chars.clone().skip_while(|c| {
        if c.is_whitespace() {
            whitespaces += 1;
            true
        } else {
            false
        }
    }).next().unwrap();

    println!("Consumed: {}", consumed);

    if consumed != expected {
        panic!("Syntax error: expected {}, found {}", expected.to_string(), consumed.to_string());
    }

    let new_contents = &contents[1 + whitespaces..];

    write_temp_file(new_contents);
}

fn peek() -> char {
    let contents = read_temp_file();

    let chars = contents.chars();

    let peek = chars.skip_while(|c| c.is_whitespace()).next().unwrap_or_else(|| ' ' as char);
    println!("peek: {}", peek);
    peek
}

pub fn syntax_analysis(source_file: &str) ->f32 {
    initialize_files(source_file);
    let result = expr();
    println!("result: {}", result.to_string());
    write_output_file(&result.to_string());

    result
}