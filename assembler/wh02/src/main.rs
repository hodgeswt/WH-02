use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use wh02_lexer::Lexer;
use wh02_lexer::position::Position;

use wh02_parser::Parser;

pub mod assembler;
pub mod assembler_error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    let output_path = &args[2];

    let start = Instant::now();

    let contents = fs::read_to_string(&input_path).expect("Failed to load input file.");
    let lexer = Lexer {
        position: Position::default(),
        characters: contents.chars().peekable(),
    };

    let mut parser = Parser::new(lexer);

    parser.parse_all();

    if parser.errors.len() > 0 {
        return;
    }

    let mut assembler = assembler::Assembler::new(parser.expressions);

    let success = assembler.assemble();

    let duration = start.elapsed();

    match success {
        Err(error) => {
            println!("ERROR: {}", error);
            return;
        },
        Ok(success) => {
            let mut output = File::create(output_path).expect("Failed to create output file.");
            write!(output, "{}", success).expect("Failed to write to output file.");
            println!("Wrote {} hex words to {}", assembler.assembled.len(), output_path);
        }
    }

    println!("Completed in {}ms ({}ns)", duration.as_millis(), duration.as_nanos());
}