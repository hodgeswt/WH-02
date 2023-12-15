use std::{env, fs};

use wh02_lexer::Lexer;
use wh02_lexer::position::Position;
use wh02_lexer::enumerations::TokenType;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let contents = fs::read_to_string(&path).expect("Failed to load input file.");
    let mut lexer = Lexer {
        position: Position::default(),
        characters: contents.chars().peekable(),
    };

    let mut token = lexer.lex();
    let mut more = true;
    while more {
        match token {
            Ok(ref t) => {
                println!("Token: '{}' Type: {}", t.value, t.token_type);
                if t.token_type == TokenType::EndOfFile {
                    more = false;
                } else {
                    token = lexer.lex();
                }
            },
            Err(ref e) => {
                println!("Token error at {}: {}", e.position, e.message);
                more = false;
            }
        }
    }
}