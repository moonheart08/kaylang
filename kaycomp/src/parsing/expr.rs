use super::Token;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinaryOp 
{
    Add,            // +
    Sub,            // -
    Multiply,       // *
    Divide,         // /
    Modulo,         // %
    BitAnd,         // &
    LogicAnd,       // &&
    BitOr,          // |
    LogicOr,        // ||
    BitXor,         // ^
    LogicXor,       // ^^
    GreaterThan,    // >
    LessThan,       // <
    GreaterThanEq,  // >=
    LessThanEq,     // <=
    BitNot,         // ~
    LogicNot,       // !
}

impl TryFrom<Token> for BinaryOp {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        use BinaryOp::*;
        'b: {
            return Ok(match value 
            {
                Token::Plus => Add,
                Token::Minus => Sub,
                Token::Multiply => Multiply,
                Token::Divide => Divide,
                Token::Modulo => Modulo,
                Token::BitAnd => BitAnd,
                Token::BitOr => BitOr,
                Token::BitXor => BitXor,
                Token::LogicAnd => LogicAnd,
                Token::LogicOr => LogicOr,
                Token::LogicXor => LogicXor,
                Token::GreaterThan => GreaterThan,
                Token::GreaterThanEq => GreaterThanEq,
                Token::LessThan => LessThan,
                Token::LessThanEq => LessThanEq,
                _ => break 'b,
            })
        }

        return Err(());
    }
}