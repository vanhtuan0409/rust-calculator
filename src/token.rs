#[derive(Debug)]
pub enum Token {
    Unknown,

    LBracket,
    RBracket,

    Add,
    Subtract,
    Mul,
    Div,

    Number(f32),
}
