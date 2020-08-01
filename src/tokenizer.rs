#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown,
    WS,

    Sep(char),

    NumberLit(f32),
    StringLit(String),
}

pub struct Tokenizer {
    pos: usize,
    input: Vec<char>,
}

impl Tokenizer {
    pub fn new(s: &str) -> Tokenizer {
        Tokenizer {
            pos: 0,
            input: s.chars().collect(),
        }
    }

    fn peak(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    fn peak_ahead(&self) -> Option<&char> {
        self.input.get(self.pos + 1)
    }

    fn scan_number(&mut self) -> Token {
        let mut buf = String::new();
        loop {
            let mut should_break = false;
            match self.peak() {
                None => should_break = true,
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
        Token::NumberLit(buf.parse().unwrap())
    }

    fn scan_string(&mut self) -> Token {
        let mut buf = String::new();
        loop {
            let mut should_break = false;
            match self.peak() {
                None => should_break = true,
                Some(c @ '1'..='9') => buf.push(*c),
                Some(c @ 'a'..='z') => buf.push(*c),
                Some(c @ 'A'..='Z') => buf.push(*c),
                Some('_') => buf.push('_'),
                Some('-') => buf.push('-'),
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
        Token::StringLit(buf)
    }

    fn scan_op(&mut self) -> Option<Token> {
        match self.peak() {
            Some('+') => Some(Token::StringLit("+".to_string())),
            Some('-') => Some(Token::StringLit("-".to_string())),
            Some('*') | Some('x') | Some('X') => Some(Token::StringLit("*".to_string())),
            Some('/') | Some(':') => Some(Token::StringLit("/".to_string())),
            Some(_) => Some(Token::Unknown),
            None => None,
        }
    }

    fn scan_ws(&mut self) -> Token {
        while let Some(' ') = self.peak_ahead() {
            self.pos += 1;
        }
        Token::WS
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = match self.peak() {
            Some(' ') => Some(self.scan_ws()),
            Some('(') => Some(Token::Sep('(')),
            Some(')') => Some(Token::Sep(')')),
            Some('0'..='9') => Some(self.scan_number()),
            Some('a'..='z') | Some('A'..='Z') => Some(self.scan_string()),
            _ => self.scan_op(),
        };
        self.pos += 1;
        token
    }
}
