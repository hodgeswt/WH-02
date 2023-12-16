use std::iter::Peekable;
use std::str::Chars;

pub mod position;
pub mod token;
pub mod enumerations;
pub mod lexer_error;

use enumerations::TokenType;
use lexer_error::LexerError;
use position::Position;
use token::Token;

fn is_newline(c: char) -> bool {
    return c == '\n' || c == '\r';
}

fn is_special(c: char) -> bool {
    return
        c == '#'
        || c == '$'
        || c == '@'
        || c == ','
        || c == ';'
        || c == '.'
        || c.is_ascii_whitespace();
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub position: Position,
    pub characters: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {

    fn next_char(&mut self) -> Option<char> {
        let c = self.characters.next();
        self.position.col += 1;
        match c {
            Some(c) => {
                if is_newline(c) {
                    self.position.line += 1;
                    self.position.col = 0;
                }
            }
            _ => {}
        }

        return c;
    }

    fn parse_hex(&mut self, val: &mut String) -> Result<(), LexerError>{
        let mut end = false;
        let mut len = 0;
        while !end {
            if len > 2 {
                return Err(LexerError {
                    message: format!("Invalid hex value: {}. Expected two hexits.", val),
                    position: self.position,
                });
            }
            let next = self.characters.peek();
            match next {
                Some(next) => {
                    if is_special(*next) {
                        break;
                    }
                },
                None => {
                    break;
                }
            }

            let c = self.next_char();
            len += 1;
            match c {
                Some(c) => {
                    if !c.is_ascii_hexdigit() {
                        return Err(LexerError {
                            message: format!("Invalid hexit: {}", c),
                            position: self.position,
                        });
                    }
                    val.push(c);
                },
                None => {
                    end = true;
                }
            }
        }

        // Normalize hex values to uppercase
        *val = val.to_ascii_uppercase();

        Ok(())
    }

    fn parse_comment(&mut self, val: &mut String) {
        let mut end = false;
        while !end {
            let next = self.characters.peek();
            match next {
                Some(next) => {
                    if is_newline(*next) {
                        break;
                    }
                },
                None => {
                    break;
                }
            }

            let c = self.next_char();
            match c {
                Some(c) => {
                    val.push(c);
                },
                None => {
                    end = true;
                }
            }
        }
    }

    fn parse_alnum(&mut self, val: &mut String) {
        let mut end = false;
        while !end {
            let next = self.characters.peek();
            match next {
                Some(next) => {
                    if !(*next).is_alphanumeric() {
                        break;
                    }
                },
                None => {
                    break;
                }
            }

            let c = self.next_char();
            match c {
                Some(c) => {
                    val.push(c);
                },
                None => {
                    end = true;
                }
            }
        }
    }

    fn parse_newline(&mut self, val: &mut String) {
        let mut end = false;
        while !end {
            let next = self.characters.peek();
            match next {
                Some(next) => {
                    if !is_newline(*next) {
                        break;
                    }
                },
                None => {
                    break;
                }
            }

            let c = self.next_char();
            match c {
                Some(c) => {
                    val.push(c);
                },
                None => {
                    end = true;
                }
            }
        }
    }

    fn parse_whitespace(&mut self, val: &mut String) {
        let mut end = false;
        while !end {
            let next = self.characters.peek();
            match next {
                Some(next) => {
                    if !(*next).is_ascii_whitespace() {
                        break;
                    }
                },
                None => {
                    break;
                }
            }

            let c = self.next_char();
            match c {
                Some(c) => {
                    val.push(c);
                },
                None => {
                    end = true;
                }
            }
        }
    }

    pub fn lex(&mut self) -> Result<Token, LexerError> {
        let mut val = String::new();
        let token_type;
        let c = self.next_char();

        let start_position = self.position.clone();

        match c {
            Some(c) => {
                if c == '.' {
                    token_type = TokenType::Word;
                    val.push(c);
                    self.parse_alnum(&mut val);
                } else if c == ',' {
                    token_type = TokenType::Comma;
                    val.push(c);
                } else if c == ';' {
                    token_type = TokenType::Comment;
                    val.push(c);
                    self.parse_comment(&mut val);
                } else if c == '#' {
                    token_type = TokenType::Hex;
                    val.push(c);
                    self.parse_hex(&mut val)?;
                } else if c == '$' {
                    token_type = TokenType::Address;
                    val.push(c);
                    self.parse_hex(&mut val)?;
                } else if c == '@' {
                    token_type = TokenType::Location;
                    val.push(c);
                    self.parse_alnum(&mut val);
                } else if c.is_alphabetic() {
                    token_type = TokenType::Operation;
                    val.push(c);
                    self.parse_alnum(&mut val);
                } else if is_newline(c) {
                    self.parse_newline(&mut val);
                    token_type = TokenType::Newline;
                    val.push(c);
                } else if c.is_ascii_whitespace() {
                    token_type = TokenType::Whitespace;
                    val.push(c);
                    self.parse_whitespace(&mut val);
                } else {
                    return Err(LexerError {
                        message: format!("Unknown character: {}", c),
                        position: self.position,
                    });
                }
            }
            None => {
                token_type = TokenType::EndOfFile;
            }
        }

        return Ok(Token::new(val, token_type, start_position));
    }
}