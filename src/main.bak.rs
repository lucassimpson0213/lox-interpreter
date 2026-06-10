use std::io::{self, Write};
use std::path::Path;

fn main() {
    if std::env::args().count() > 2 {
        println!("Usage: jlox [script]");
        println!("number of command line args: {}", std::env::args().count());
    } else if std::env::args().count() == 2 {
        //TODO run script
        let file_arg = std::env::args().nth(1);
        match file_arg {
            Some(arg) => {
                run_file(&arg);

                println!("number of command line args: {}", std::env::args().count());
            }
            None => {
                eprintln!("malformed file argument");
                eprintln!("Usage: jlox [script]");
                println!("number of command line args: {}", std::env::args().count());
            }
        }
    } else {
        println!("number of command line args: {}", std::env::args().count());
        run_prompt();
    }
}

fn run_file(file: &str) {
    let contents = std::fs::read_to_string(file);
}

fn run(source: &String) {}
fn run_prompt() {
    let mut input_string = String::new();
    loop {
        print!("<lox> -> ");
        io::stdout().flush().unwrap();

        let read_line = std::io::stdin().read_line(&mut input_string).unwrap();
        run(&input_string);
        input_string.clear();
        println!("{}", input_string);
    }
}

enum TokenType {
    left_paren,
    right_paren,
    left_brace,
    right_brace,
    comma,
    dot,
    minus,
    plus,
    semicolon,
    slash,
    star,

    // One or two character tokens.
    bang,
    bang_equal,
    equal,
    equal_equal,
    greater,
    greater_equal,
    less,
    less_equal,

    // Literals.
    identifier,
    string,
    number,

    // Keywords.
    and,
    class,
    Else,
    False,
    fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
