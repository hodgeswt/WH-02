use wh02_parser::expressions::Expressions;
use wh02_parser::keyword::Keyword;

use crate::assembler_error::AssemblerError;

pub fn assemble_binary_expression(expr: Expressions) -> Result<String, AssemblerError> {
    match expr {
        Expressions::BinaryExpression { .. } => {
            Ok("".to_string())
        },
        _ => {
            Err(
                AssemblerError {
                    message: "Found unexpected expression type. How did we get here?".to_string()
                }
            )
        }
    }
}

pub fn assemble_unary_expression(expr: Expressions) -> Result<String, AssemblerError> {
    match expr {
        Expressions::UnaryExpression { keyword, .. } => {
            match keyword {
                Keyword::DEF => Ok("DEF".to_string()),
                Keyword::START => Ok("START".to_string()),
                _ => Err(
                    AssemblerError {
                        message: "Found unexpected keyword. How did we get here?".to_string()
                    }
                )
            }
        },
        _ => {
            Err(
                AssemblerError {
                    message: "Found unexpected expression type. How did we get here?".to_string()
                }
            )
        }
    }
}

pub fn assemble_no_operand_expression(expr: Expressions) -> Result<String, AssemblerError> {
    match expr {
        Expressions::NoOperandExpression { keyword } => {
            match keyword {
                Keyword::HLT => Ok("0020".to_string()),
                Keyword::NOP => Ok("0000".to_string()),
                _ => Err(
                    AssemblerError {
                        message: "Found unexpected keyword. How did we get here?".to_string()
                    }
                )

            }
        },
        _ => {
            Err(
                AssemblerError {
                    message: "Found unexpected expression type. How did we get here?".to_string()
                }
            )
        }
    }
}