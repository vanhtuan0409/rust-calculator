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
