use std::collections::HashMap;

use wh02_parser::expressions::Expressions;
use wh02_parser::keyword::Keyword;

use crate::assembler_error::AssemblerError;

#[derive(Debug, Clone)]
pub struct Assembler {
    pub expressions: Vec<Expressions>,
    start_index: usize,
    size: usize,
    pub words: HashMap<String, usize>,
    index: usize,
}

impl Assembler {

    pub fn new(expressions: Vec<Expressions>) -> Self {
        Assembler {
            expressions,
            start_index: 0,
            size: 256,
            words: HashMap::new(),
            index: 0,
        }
    }

    pub fn assemble(&mut self) -> Result<Vec<String>, AssemblerError> {
        let mut assembled: Vec<String> = vec!["0000".to_string(); self.size];
        self.index = self.start_index;

        for expr in self.expressions.clone() {
            let result = self.assemble_expression(expr.clone())?;

            if result != "" {
                assembled[self.index] = result;
                self.index += 1;
            }
        }

        Ok(assembled)
    }

    pub fn assemble_expression(&mut self, expr: Expressions) -> Result<String, AssemblerError> {
        match expr {
            Expressions::NoOperandExpression { .. } => {
                self.assemble_no_operand_expression(expr)
            },
            Expressions::UnaryExpression { .. } => {
                self.assemble_unary_expression(expr)
            },
            Expressions::BinaryExpression { .. } => {
                self.assemble_binary_expression(expr)
            }
        }
    }

    fn assemble_binary_expression(&mut self, expr: Expressions) -> Result<String, AssemblerError> {
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

    fn assemble_unary_expression(&mut self, expr: Expressions) -> Result<String, AssemblerError> {
        match expr {
            Expressions::UnaryExpression { keyword, operand } => {
                match keyword {
                    Keyword::DEF =>  {
                        // Not actual code for the processor, but sets
                        // a label to a memory address
                        self.words.insert(operand.value, self.index);
                        Ok("".to_string())
                    },
                    Keyword::START => {
                        // Not actual code for the processor, but sets
                        // where we start in memory
                        self.start_index = usize::from_str_radix(&operand.value, 16)
                            .expect(&format!("Expected valid hex value in 32bit range; found {}", operand.value));
                        self.index = self.start_index;
                        Ok("".to_string())
                    },
                    Keyword::JMP => {
                        Ok("".to_string())
                    }
                    _ => {
                        Err(
                            AssemblerError {
                                message: format!("Found unexpected keyword {}. How did we get here?", keyword)
                            }
                        )
                    }
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

    fn assemble_no_operand_expression(&mut self, expr: Expressions) -> Result<String, AssemblerError> {
        match expr {
            Expressions::NoOperandExpression { keyword } => {
                match keyword {
                    Keyword::HLT => Ok("0020".to_string()),
                    Keyword::NOP => Ok("0000".to_string()),
                    _ => Err(
                        AssemblerError {
                            message: format!("Found unexpected keyword {}. How did we get here?", keyword)
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
}