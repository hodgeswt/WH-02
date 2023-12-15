use crate::keyword::Keyword;
use crate::operand::Operand;
use crate::parser_error::ParserError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expressions {
    NoOperandExpression {
        keyword: Keyword,
    },

    UnaryExpression {
        keyword: Keyword,
        operand: Operand,
    },

    BinaryExpression {
        keyword: Keyword,
        operand1: Operand,
        comma: String,
        operand2: Operand,
    }
}

impl Expressions {
    pub fn validate_no_operand_keyword(keyword: Keyword) -> Result<(), ParserError> {
        match keyword {
            Keyword::HLT => Ok(()),
            Keyword::NOP => Ok(()),
            _ => Err(ParserError {
                position: Default::default(),
                message: format!("\n\t==> Invalid keyword: {}. Expected one of {:#?}", keyword, vec![Keyword::HLT, Keyword::NOP]),
            }),
        }
    }

    pub fn validate_unary_keyword(keyword: Keyword) -> Result<(), ParserError> {
        match keyword {
            Keyword::DEF => Ok(()),
            Keyword::START => Ok(()),
            _ => Err(ParserError {
                position: Default::default(),
                message: format!("\n\t==> Invalid keyword: {}. Expected one of {:#?}", keyword, vec![Keyword::DEF, Keyword::START]),
            }),
        }
    }

    pub fn validate_binary_keyword(keyword: Keyword) -> Result<(), ParserError> {
        match keyword {
            Keyword::MOV => Ok(()),
            _ => Err(ParserError {
                position: Default::default(),
                message: format!("\n\t==> Invalid keyword: {}. Expected one of {:#?}", keyword, vec![Keyword::MOV]),
            }),
        }
    }
}