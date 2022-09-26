use std::fs::File;
use std::io::Read;
use std::io::Write;

const SOURCE_FILE: &str = "example/test.op";
const TEMP_FILE: &str = "example/tes.tmp";
const DEST_FILE: &str = "example/test.res";

fn expr() -> i32 {
    let term_result = term();
    expr_tail( term_result)
}

fn expr_tail(inherited: i32) -> i32 {
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

fn term() -> i32 {
    term_tail(factor())
}

fn term_tail( inherited: i32) -> i32 {
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

fn factor() -> i32 {
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

fn digit() -> i32 {
    let peek = peek();
    match peek {
        '0' => {
            consume('0');
            0
        },
        '1' => {
            consume('1');
            1
        },
        '2' => {
            consume('2');
            2
        },
        '3' => {
            consume('3');
            3
        },
        '4' => {
            consume('4');
            4
        },
        '5' => {
            consume('5');
            5
        },
        '6' => {
            consume('6');
            6
        },
        '7' => {
            consume('7');
            7
        },
        '8' => {
            consume('8');
            8
        },
        '9' => {
            consume('9');
            9
        },
        _ => panic!("Unexpected character: {}", peek)
    }
}

fn initialize_files() {
    let mut file = File::open(SOURCE_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut file = File::create(TEMP_FILE).unwrap();
    write!(file, "{}", contents).unwrap();
}

fn read_temp_file() -> String {
    let mut file = File::open(TEMP_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn consume(expected: char) {
    let contents = read_temp_file();
    let mut file = File::create(TEMP_FILE).unwrap();

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

    println!("New contents: {}", new_contents);

    write!(file, "{}", new_contents).unwrap();
}

fn peek() -> char {
    let contents = read_temp_file();

    let chars = contents.chars();

    let peek = chars.skip_while(|c| c.is_whitespace()).next().unwrap_or_else(|| ' ' as char);
    println!("peek: {}", peek);
    peek
}

pub fn syntax_analysis() ->i32 {
    initialize_files();
    let result = expr();
    println!("result: {}", result.to_string());

    let mut dest_file = File::create(DEST_FILE).unwrap();
    write!(dest_file, "{}", result.to_string()).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use crate::{syntax_analysis};

    #[test]
    fn it_works() {
        assert_eq!(1, syntax_analysis());
    }
}
