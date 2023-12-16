use std::collections::HashMap;

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
    pub errors: Vec<ParserError>,
    pub has_next: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer,
            expressions: Vec::new(),
            errors: Vec::new(),
            has_next: true,
        }
    }

    fn validate_token_types(&mut self, tokens: &Vec<Token>, types: Vec<Vec<TokenType>>) -> Result<(), ParserError> {
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

    fn parse_no_operand(&mut self, toks: Vec<Token>) -> Result<(), ParserError> {
        let token_types = vec![
            vec![TokenType::Operation],
            vec![TokenType::Newline],
        ];
        self.validate_token_types(&toks, token_types)?;

        let result = Expressions::validate_no_operand_keyword(
            Keyword::from_str(&toks[0].value, toks[0].start_position)?
        );

        match result {
            Ok(_) => {},
            Err(mut error) => {
                error.position = toks[0].start_position;
                return Err(error);
            }
        }

        let keyword = toks[0].value.to_string();
        self.expressions.push(Expressions::NoOperandExpression {
            keyword: Keyword::from_str(&keyword, toks[0].start_position)?,
        });

        Ok(())
    }

    fn parse_unary(&mut self, toks: Vec<Token>) -> Result<(), ParserError> {
        let keyword = Keyword::from_str(&toks[0].value, toks[0].start_position)?;

        let keyword_operands: HashMap<Keyword, Vec<TokenType>> = HashMap::from([
            (Keyword::DEF, vec![TokenType::Word]),
            (Keyword::START, vec![TokenType::Address]),
            (Keyword::JMP, vec![TokenType::Address, TokenType::Word]),
        ]);

        let token_types = vec![
            vec![TokenType::Operation],
            keyword_operands[&keyword].clone(),
            vec![TokenType::Newline],
        ];

        self.validate_token_types(&toks, token_types)?;

        let result = Expressions::validate_unary_keyword(keyword.clone());

        if keyword == Keyword::START && self.expressions.len() > 0 {
            return Err(ParserError {
                message: "START instruction must occur first.".to_string(),
                position: toks[0].start_position,
            })
        } else if keyword == Keyword::START {

        }

        match result {
            Ok(_) => {},
            Err(mut error) => {
                error.position = toks[0].start_position;
                return Err(error);
            }
        }

        let operand = toks[1].value.to_string();

        self.expressions.push(Expressions::UnaryExpression {
            keyword: keyword.clone(),
            operand: Operand::from_str(&operand, toks[1].start_position)?,
        });

        Ok(())
    }

    fn parse_binary(&mut self, toks: Vec<Token>) -> Result<(), ParserError> {
        let keyword = Keyword::from_str(&toks[0].value, toks[0].start_position)?;

        let keyword_operands: HashMap<Keyword, Vec<TokenType>> = HashMap::from([
            (Keyword::MOV, vec![TokenType::Hex, TokenType::Address, TokenType::Location]),
        ]);

        let token_types = vec![
            vec![TokenType::Operation],
            keyword_operands[&keyword].clone(),
            vec![TokenType::Comma],
            vec![
                TokenType::Hex,
                TokenType::Address,
                TokenType::Location,
                TokenType::Word,
            ],
            vec![TokenType::Newline],
        ];
        self.validate_token_types(&toks, token_types)?;

        let result = Expressions::validate_binary_keyword(
            Keyword::from_str(&toks[0].value, toks[0].start_position)?
        );

        match result {
            Ok(_) => {},
            Err(mut error) => {
                error.position = toks[0].start_position;
                return Err(error);
            }
        }

        let keyword = toks[0].value.to_string();
        let operand1 = toks[1].value.to_string();
        let operand2 = toks[3].value.to_string();

        self.expressions.push(Expressions::BinaryExpression {
            keyword: Keyword::from_str(&keyword, toks[0].start_position)?,
            operand1: Operand::from_str(&operand1, toks[1].start_position)?,
            comma: toks[2].value.to_string(),
            operand2: Operand::from_str(&operand2, toks[3].start_position)?,
        });

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
                // End of File
                return Ok(())
            }
            1 => {
                let res = self.parse_no_operand(toks);
                match res {
                    Ok(_) => {},
                    Err(error) => {
                        self.errors.push(error);
                    }
                }
            }
            2 => {
                let res = self.parse_no_operand(toks);
                match res {
                    Ok(_) => {},
                    Err(error) => {
                        self.errors.push(error);
                    }
                }
            },

            3 => {
                let res = self.parse_unary(toks);
                match res {
                    Ok(_) => {},
                    Err(error) => {
                        self.errors.push(error);
                    }
                }
            },

            5 => {
                let res = self.parse_binary(toks);
                match res {
                    Ok(_) => {},
                    Err(error) => {
                        self.errors.push(error);
                    }
                }
            },
            _ => {
                self.errors.push(ParserError {
                    message: format!("{:#?}\nInvalid expression length. Expected 2, 3, or 5. Got: {}", toks, toks.len()),
                    position: toks[0].start_position,
                });
            }
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        let line: Result<Vec<Token>, ParserError> = self.get_line();
        match line {
            Ok(line) => {
                self.parse_line(line)?;
            },
            Err(error) => {
                return Err(error);
            }
        }

        Ok(())
    }

    fn validate_location(&self, token: &Token) -> Result<(), ParserError> {
        let valid_locations = vec![
            "@A",
            "@B",
            "@C",
            "@O1",
            "@O2",
            "@ACC"
        ];

        if !valid_locations.contains(&token.value.as_str()) {
            return Err(ParserError {
                position: token.start_position,
                message: format!("Invalid location provided: {}. Expected one of {:#?}", token.value, valid_locations),
            })
        }

        Ok(())
    }

    fn get_line(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut end = false;
        let mut line: Vec<Token> = Vec::new();
        while !end {
            let token: Result<Token, LexerError> = self.lexer.lex();

            match token {
                Ok (token) => {
                    if token.token_type == TokenType::EndOfFile {
                        self.has_next = false;
                        end = true;
                    }

                    if token.token_type == TokenType::Location {
                        let res = self.validate_location(&token);
                        match res {
                            Ok(_) => {},
                            Err(error) => {
                                self.errors.push(error);
                            }
                        }
                    }

                    line.push(token.clone());
                    if token.token_type == TokenType::Newline {
                        end = true;
                    }
                },
                Err(error) => {
                    return Err(ParserError {
                        position: error.position,
                        message: format!("\n\t==> Lexical error: {}", error.message),
                    });
                },
            }
        }

        Ok(line)
    }
}