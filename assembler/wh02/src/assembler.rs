use wh02_parser::expressions::Expressions;
use wh02_parser::keyword::Keyword;

use crate::assembler_error::AssemblerError;

pub fn assemble_expression(expr: Expressions, start_index: &mut u32) -> Result<String, AssemblerError> {
    match expr {
        Expressions::NoOperandExpression { .. } => {
            assemble_no_operand_expression(expr)
        },
        Expressions::UnaryExpression { .. } => {
            assemble_unary_expression(expr, start_index)
        },
        Expressions::BinaryExpression { .. } => {
            assemble_binary_expression(expr)
        }
    }
}

fn assemble_binary_expression(expr: Expressions) -> Result<String, AssemblerError> {
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

fn assemble_unary_expression(expr: Expressions, start_index: &mut u32) -> Result<String, AssemblerError> {
    match expr {
        Expressions::UnaryExpression { keyword, operand } => {
            match keyword {
                Keyword::DEF => Ok("DEF".to_string()),
                Keyword::START => {
                    // Not actual code for the processor, but sets
                    // where we start in memory
                    *start_index = u32::from_str_radix(&operand.value, 16)
                        .expect(&format!("Expected valid hex value in 32bit range; found {}", operand.value));
                    Ok("".to_string())
                },
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

fn assemble_no_operand_expression(expr: Expressions) -> Result<String, AssemblerError> {
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