use crate::keyword::Keyword;
use crate::operand::Operand;

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