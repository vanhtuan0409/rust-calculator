use crate::tokenizer::Tokenizer;

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Expression {
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    Grouping(Box<Expression>),
}

pub struct BinaryExpression {
    op: Op,
    lhs: Expression,
    rhs: Expression,
}

pub struct UnaryExpression {
    op: Op,
    exp: Expression,
}

pub struct AstParser {
    tokens: Tokenizer,
}

pub enum AstParseError {
    InvalidToken,
}

impl AstParser {
    pub fn new(tokens: Tokenizer) -> AstParser {
        AstParser { tokens }
    }

    pub fn parse(&mut self) -> Result<Expression, AstParseError> {
        Err(AstParseError::InvalidToken)
    }
}
