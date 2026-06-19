use std::io::{self, Write};

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
    println!("contents of file: {:?}", contents);

    let unwrapped_contents = contents.expect("Something went wrong when parsing the string");

    let lines = unwrapped_contents.lines();

    for line in lines.into_iter() {
        run(line);
    }
}

fn run(source: &str) {}
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

pub mod lexer {

    pub struct Scanner {
        line: Vec<char>,
        char_ptr: usize,
        tokens: Vec<Token>,
    }

    impl Scanner {
        pub fn new(line: String) -> Self {
            Self {
                line: line.chars().collect(),
                tokens: Vec::new(),
                char_ptr: 0,
            }
        }

        fn advance(self) -> Result<char, std::io::Error> {
            let c = self.line.get(self.char_ptr);

            match c {
                Some(character) => {
                    return Ok(*character);
                }
                None => Err(()),
            }

            match c {
                ';' => {}
                _ => {
                    println!("default case");
                }
            }
            //basically check for one character tokens first to make sure you can match those
            //then expand

            //if not at end increment char_ptr
            //
            //
            //return the character
        }

        fn is_at_end() {
            //check to see if char_ptr matches the ending of the collection
        }
        pub fn scan_tokens(self) {}
    }

    struct Token {
        kind: TokenType,
        lexeme: String,
        line: usize,
    }
    enum TokenType {
        //MARKER
        leftparen,
        right_paren,
        left_brace,
        RightBrace,
        Comma,
        dot,
        //MARKER
        minus,
        Plus,
        semicolon,
        Slash,
        Star,
        // One or two character tokens.
        bang,
        BangEqual,
        Equal,
        equal_equal,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,

        // Literals.
        Identifier,
        String,
        Number,

        // Keywords.
        And,
        Class,
        Else,
        False,
        Fun,
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
}
