use wh02_lexer::Lexer;
use wh02_lexer::token::Token;
use wh02_lexer::enumerations::TokenType;
use wh02_lexer::lexer_error::LexerError;

pub mod keyword;
pub mod expressions;
pub mod operand;
pub mod parser_error;

use crate::expressions::Expressions;
use crate::parser_error::ParserError;
use crate::keyword::Keyword;
use crate::operand::Operand;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub expressions: Vec<Expressions>,
    pub has_next: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer,
            expressions: Vec::new(),
            has_next: true,
        }
    }

    fn match_token_types(&mut self, tokens: &Vec<Token>, types: Vec<Vec<TokenType>>) -> Result<(), ParserError> {
        for i in 0..tokens.len() {
            if !types[i].contains(&tokens[i].token_type) {
                return Err(ParserError {
                    message: format!("Invalid token type {}, expected one of {:#?}", tokens[i].token_type, types[i]),
                    position: tokens[i].start_position,
                })
            }
        }

        Ok(())
    }

    fn parse_line(&mut self, line: Vec<Token>) -> Result<(), ParserError> {
        let plain_tokens: Vec<Token> = line.clone();
        let toks: Vec<Token> = plain_tokens
            .iter()
            .filter(
                |x|
                x.token_type != TokenType::Whitespace
                && x.token_type != TokenType::Comment
                && x.token_type != TokenType::EndOfFile
            )
            .cloned()
            .collect();

        match toks.len() {
            0 => {
                return Ok(())
            }
            2 => {
                let token_types = vec![
                    vec![TokenType::Operation],
                    vec![TokenType::Newline],
                ];
                self.match_token_types(&toks, token_types)?;

                let keyword = toks[0].value.to_string();
                self.expressions.push(Expressions::NoOperandExpression {
                    keyword: Keyword::from_str(&keyword, toks[0].start_position)?,
                });
            },

            3 => {
                let token_types = vec![
                    vec![TokenType::Operation],
                    vec![
                        TokenType::Hex,
                        TokenType::Address,
                        TokenType::Location,
                    ],
                    vec![TokenType::Newline],
                ];
                self.match_token_types(&toks, token_types)?;

                let keyword = toks[0].value.to_string();
                let operand = toks[1].value.to_string();

                self.expressions.push(Expressions::UnaryExpression {
                    keyword: Keyword::from_str(&keyword, toks[0].start_position)?,
                    operand: Operand::from_str(&operand, toks[1].start_position)?,
                });
            },

            5 => {
                let token_types = vec![
                    vec![TokenType::Operation],
                    vec![
                        TokenType::Hex,
                        TokenType::Address,
                        TokenType::Location,
                    ],
                    vec![TokenType::Comma],
                    vec![
                        TokenType::Hex,
                        TokenType::Address,
                        TokenType::Location,
                    ],
                    vec![TokenType::Newline],
                ];
                self.match_token_types(&toks, token_types)?;

                let keyword = toks[0].value.to_string();
                let operand1 = toks[1].value.to_string();
                let operand2 = toks[3].value.to_string();

                self.expressions.push(Expressions::BinaryExpression {
                    keyword: Keyword::from_str(&keyword, toks[0].start_position)?,
                    operand1: Operand::from_str(&operand1, toks[1].start_position)?,
                    comma: toks[2].value.to_string(),
                    operand2: Operand::from_str(&operand2, toks[3].start_position)?,
                });
            },
            _ => {
                return Err(ParserError {
                    message: format!("{:#?}\nInvalid expression length. Expected 2, 3, or 5. Got: {}", toks, toks.len()),
                    position: toks[0].start_position,
                })
            }
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        let line: Result<Vec<Token>, LexerError> = self.get_line();
        match line {
            Ok(line) => {
                self.parse_line(line)?;
            },
            Err(error) => {
                return Err(ParserError {
                    position: error.position,
                    message: format!("Lexical error: {}", error.message),
                });
            }
        }

        Ok(())
    }

    fn get_line(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut end = false;
        let mut line: Vec<Token> = Vec::new();
        while !end {
            let token: Token = self.lexer.lex()?;
            if token.token_type == TokenType::EndOfFile {
                self.has_next = false;
                end = true;
            }

            line.push(token.clone());
            if token.token_type == TokenType::Newline {
                end = true;
            }
        }

        Ok(line)
    }
}