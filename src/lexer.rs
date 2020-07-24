use crate::token::Token;

pub struct Lexer {
    pos: usize,
    input: Vec<char>,
}

impl Lexer {
    pub fn new(s: &str) -> Lexer {
        Lexer {
            pos: 0,
            input: s.chars().collect(),
        }
    }

    fn peak(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    fn scan_number(&mut self) -> Token {
        let mut buf = String::new();
        loop {
            let mut should_break = false;
            match self.peak() {
                None | Some(' ') => should_break = true,
                Some(digit @ '1'..='9') => buf.push(*digit),
                Some('.') => buf.push('.'),
                Some(_) => {
                    should_break = true;
                    self.pos -= 1;
                }
            }

            if should_break {
                break;
            }
            self.pos += 1;
        }
        Token::Number(buf.parse().unwrap())
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // skip whitespace
        while let Some(' ') = self.peak() {
            self.pos += 1;
        }

        let token = match self.peak() {
            Some('(') => Some(Token::LBracket),
            Some(')') => Some(Token::RBracket),
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') | Some('x') | Some('X') => Some(Token::Mul),
            Some('/') | Some(':') => Some(Token::Div),
            Some('0'..='9') => Some(self.scan_number()),
            Some(_) => Some(Token::Unknown),
            None => None,
        };
        self.pos += 1;
        token
    }
}
