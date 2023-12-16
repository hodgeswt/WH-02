use std::{env, fs};

use wh02_lexer::Lexer;
use wh02_lexer::position::Position;

use wh02_parser::Parser;

pub mod assembler;
pub mod assembler_error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let contents = fs::read_to_string(&path).expect("Failed to load input file.");
    let lexer = Lexer {
        position: Position::default(),
        characters: contents.chars().peekable(),
    };

    let mut parser = Parser::new(lexer);
    while parser.has_next {
        let result = parser.parse();

        match result {
            Err(error) => {
                println!("ERROR: {}", error);
            },
            Ok(_) => { }
        }
    }

    if parser.errors.len() > 0 {
        for error in parser.errors {
            println!("ERROR: {}", error);
        }

        return;
    }

    let mut assembler = assembler::Assembler::new(parser.expressions);

    let success = assembler.assemble();

    match success {
        Err(error) => {
            println!("ERROR: {}", error);
            return;
        },
        Ok(success) => {
            println!("{:?}", success);
        }

    }


}