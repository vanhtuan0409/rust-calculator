#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown,
    WS,

    Sep(char),
    Op(char),

    Number(f32),
}

pub const LBRACKET: Token = Token::Sep('(');
pub const RBRACKET: Token = Token::Sep(')');

pub const ADD: Token = Token::Op('+');
pub const SUB: Token = Token::Op('-');
pub const MUL: Token = Token::Op('*');
pub const DIV: Token = Token::Op('/');

trait Expression {
    fn evaluate(&self) -> i32;
}
