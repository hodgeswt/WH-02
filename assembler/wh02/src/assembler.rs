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
    pub assembled: Vec<String>,
}

impl Assembler {

    pub fn new(expressions: Vec<Expressions>) -> Self {
        Assembler {
            expressions,
            start_index: 0,
            size: 256,
            words: HashMap::new(),
            index: 0,
            assembled: Vec::new(),
        }
    }

    pub fn assemble(&mut self) -> Result<String, AssemblerError> {
        self.assembled = vec!["00".to_string(); self.size];
        self.index = self.start_index;

        for expr in self.expressions.clone() {
            let result = self.assemble_expression(expr.clone())?;

            if result != "" {
                let splits = result.split(' ').collect::<Vec<&str>>();
                for entry in splits {
                    self.assembled[self.index] = entry.to_string();
                    self.index += 1;
                }
            }
        }

        let mut output = String::new();
        output += "v3.0 hex words addressed\n00: ";
        let mut counter = 0;
        let mut address = 0;
        let len = self.assembled.len();
        for byte in self.assembled.clone() {
            output += format!("{} ", byte).as_str();
            counter += 1;
            address += 1;
            if counter % 16 == 0 && address < len {
                output += format!("\n{:02x}: ", address).as_str();
            }
        }

        Ok(output)
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
            Expressions::BinaryExpression { keyword, operand1, operand2, .. } => {
                match keyword {
                    Keyword::MOV => {
                        let mut result = "".to_string();
                        if operand1.indicator == '#' {
                            if operand2.indicator == '$' {
                                result += "26";
                            } else {
                                match operand2.value.as_str() {
                                    "A" => {
                                        result += "21";
                                    }
                                    "B" => {
                                        result += "22";
                                    }
                                    "C" => {
                                        result += "23";
                                    }
                                    "O1" => {
                                        result += "24";
                                    }
                                    "O2" => {
                                        result += "25";
                                    }
                                    _ => {
                                        return Err(
                                            AssemblerError {
                                                message: format!("Found unexpected operand {}. How did we get here?", operand2.value)
                                            }
                                        )
                                    }
                                }
                            }

                            result += format!(" {}", operand1.value).as_str();
                        } else if operand1.indicator == '$' {
                            if operand2.indicator == '$' {
                                result += "1F";
                            } else {
                                match operand2.value.as_str() {
                                    "A" => {
                                        result += "1A";
                                    }
                                    "B" => {
                                        result += "1B";
                                    }
                                    "C" => {
                                        result += "1C";
                                    }
                                    "O1" => {
                                        result += "1D";
                                    }
                                    "O2" => {
                                        result += "1E";
                                    }
                                    _ => {
                                        return Err(
                                            AssemblerError {
                                                message: format!("Found unexpected operand {}. How did we get here?", operand2.value)
                                            }
                                        )
                                    }
                                }
                            }
                        result += format!(" {}", operand1.value).as_str();
                        } else {
                            match (operand1.value.as_str(), operand2.value.as_str()) {
                                ("A", "B") => {
                                    result += "01";
                                },
                                ("A", "C") => {
                                    result += "02";
                                },
                                ("A", "O1") => {
                                    result += "03";
                                }
                                ("A", "O2") => {
                                    result += "04";
                                },
                                ("B", "A") => {
                                    result += "06";
                                },
                                ("B", "C") => {
                                    result += "07";
                                },
                                ("B", "O1") => {
                                    result += "08";
                                },
                                ("B", "O2") => {
                                    result += "09";
                                },
                                ("C", "A") => {
                                    result += "0B";
                                }
                                ("C", "B") => {
                                    result += "0C"
                                },
                                ("C", "O1") => {
                                    result += "0D";
                                },
                                ("C", "O2") => {
                                    result += "0E";
                                },
                                ("O1", "A") => {
                                    result += "10";
                                },
                                ("O1", "B") => {
                                    result += "11";
                                },
                                ("O1", "C") => {
                                    result += "12";
                                },
                                ("O1", "O2") => {
                                    result += "13";
                                },
                                ("O2", "A") => {
                                    result += "15";
                                },
                                ("O2", "B") => {
                                    result += "16";
                                },
                                ("O2", "C") => {
                                    result += "17";
                                },
                                ("O2", "O1") => {
                                    result += "18";
                                },
                                ("ACC", "A") => {
                                    result += "27";
                                },
                                ("ACC", "B") => {
                                    result += "28";
                                },
                                ("ACC", "C") => {
                                    result += "29";
                                },
                                ("ACC", "O1") => {
                                    result += "2A";
                                },
                                ("ACC", "O2") => {
                                    result += "2B";
                                },
                                _ => {
                                    return Err(
                                        AssemblerError {
                                            message: format!("Found unexpected operand combination {} and {}. How did we get here?", operand1.value, operand2.value)
                                        }
                                    )
                                }
                            }
                        }
                        Ok(result)
                    }
                    _ => {
                        Ok("".to_string())
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
                    Keyword::HLT => Ok("20".to_string()),
                    Keyword::NOP => Ok("00".to_string()),
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