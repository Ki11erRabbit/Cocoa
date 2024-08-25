use std::str::CharIndices;

pub trait LexerIterator<'a> {
    fn next_token(&mut self) -> LexerResult<'a>;
}

pub type LexerResult<'a> = Result<SpannedToken<'a>, LexerError>;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    Error {
        message: String,
        start: usize,
        end: usize,
    },
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct SpannedToken<'a> {
    pub token: Token<'a>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Label(&'a str),
    TypeIdentifier(&'a str),
    Identifier(&'a str),
    U8Lit(u8),
    U16Lit(u16),
    U32Lit(u32),
    U64Lit(u64),
    I8Lit(i8),
    I16Lit(i16),
    I32Lit(i32),
    I64Lit(i64),
    IntLit(i128),
    F32Lit(f32),
    F64Lit(f64),
    FloatLit(f64),
    CharLit(char),
    StringLit(String),
    BoolLit(bool),
    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    Comma,
    Colon,
    Scope,
    Semicolon,
    Arrow,
    FatArrow,
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Not,
    LShift,
    RShift,
    LogAnd,
    LogOr,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    Dot,
    ExclusiveRange,
    InclusiveRange,
    Try,
    If,
    Else,
    While,
    For,
    In,
    Loop,
    Break,
    Continue,
    Return,
    Match,
    Is,
    Let,
    As,
    Fn,
    Trait,
    Struct,
    Enum,
    Pub,
    Priv,
    Self_,
    SelfType,
    Impl,
}
    

pub struct Lexer<'a> {
    raw_input: &'a str,
    input: std::iter::Peekable<CharIndices<'a>>,
    newline_pos: Vec<usize>,
    peeked: Option<LexerResult<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            raw_input: input,
            input: input.char_indices().peekable(),
            newline_pos: vec![0],
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&LexerResult<'a>> {
        if self.peeked.is_none() {
            self.peeked = Some(self.next_token());
        }
        self.peeked.as_ref()
    }

    fn get_next(&mut self) -> Option<(usize, char)> {
        self.input.next()
    }

    fn peek_next(&mut self) -> Option<&(usize, char)> {
        self.input.peek()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(peeked) = self.peeked.take() {
            Some(peeked)
        } else {
            Some(self.next_token())
        }
    }
}


impl<'a> LexerIterator<'a> for Lexer<'a> {

    fn next_token(&mut self) -> LexerResult<'a> {
        let mut end;
        let start;
        let token = match self.get_next() {
            Some((startt, c)) => {
                start = startt;
                end = start;
                match c {
                    'a'..='z' | '_' => {
                        while let Some((i, c)) = self.peek_next() {
                            if c.is_alphanumeric() || *c == '_' {
                                end = *i;
                                self.get_next();
                            } else {
                                break;
                            }
                        }
                        let ident = &self.raw_input[start..=end];
                        match ident {
                            "u8" => Token::TypeIdentifier(ident),
                            "u16" => Token::TypeIdentifier(ident),
                            "u32" => Token::TypeIdentifier(ident),
                            "u64" => Token::TypeIdentifier(ident),
                            "i8" => Token::TypeIdentifier(ident),
                            "i16" => Token::TypeIdentifier(ident),
                            "i32" => Token::TypeIdentifier(ident),
                            "i64" => Token::TypeIdentifier(ident),
                            "f32" => Token::TypeIdentifier(ident),
                            "f64" => Token::TypeIdentifier(ident),
                            "char" => Token::TypeIdentifier(ident),
                            "bool" => Token::TypeIdentifier(ident),
                            "self" => Token::Self_,
                            "true" => Token::BoolLit(true),
                            "false" => Token::BoolLit(false),
                            "if" => Token::If,
                            "else" => Token::Else,
                            "while" => Token::While,
                            "for" => Token::For,
                            "in" => Token::In,
                            "loop" => Token::Loop,
                            "break" => Token::Break,
                            "continue" => Token::Continue,
                            "return" => Token::Return,
                            "match" => Token::Match,
                            "is" => Token::Is,
                            "let" => Token::Let,
                            "as" => Token::As,
                            "fn" => Token::Fn,
                            "trait" => Token::Trait,
                            "struct" => Token::Struct,
                            "enum" => Token::Enum,
                            "pub" => Token::Pub,
                            "priv" => Token::Priv,
                            "impl" => Token::Impl,
                            _ => Token::Identifier(ident),
                        }
                    },
                    'A'..='Z' => {
                        while let Some((i, c)) = self.peek_next() {
                            if c.is_alphanumeric() || *c == '_' {
                                end = *i;
                                self.get_next();
                            } else {
                                break;
                            }
                        }
                        match &self.raw_input[start..=end] {
                            "Self" => Token::SelfType,
                            ident => Token::TypeIdentifier(ident),
                        }
                    },
                    '0'..='9' => {
                        while let Some((i, c)) = self.peek_next() {
                            if c.is_digit(10) {
                                end = *i;
                                self.get_next();
                            } else {
                                break;
                            }
                        }
                        let digit_end = end;
                        if let Some((_, 'u')) = self.peek_next() {
                            self.get_next();
                            let mut suffix = String::with_capacity(2);
                            while let Some((i, c)) = self.peek_next() {
                                if c.is_digit(10) {
                                    suffix.push(*c);
                                    end = *i;
                                    self.get_next();
                                } else {
                                    break;
                                }
                            }
                            match suffix.as_str() {
                                "8" => Token::U8Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                "16" => Token::U16Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                "32" => Token::U32Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                "64" => Token::U64Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                _ => return Err(LexerError::Error {
                                    message: "Invalid unsigned integer suffix".to_string(),
                                    start,
                                    end,
                                }),
                            }
                            
                        } else if let Some((_, 'i')) = self.peek_next() {
                            self.get_next();
                            let mut suffix = String::with_capacity(2);
                            while let Some((i, c)) = self.peek_next() {
                                if c.is_digit(10) {
                                    suffix.push(*c);
                                    end = *i;
                                    self.get_next();
                                } else {
                                    break;
                                }
                            }
                            match suffix.as_str() {
                                "8" => Token::I8Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                "16" => Token::I16Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                "32" => Token::I32Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                "64" => Token::I64Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                _ => return Err(LexerError::Error {
                                    message: "Invalid signed integer suffix".to_string(),
                                    start,
                                    end,
                                }),
                            }
                        } else if let Some((_, '.')) = self.peek_next() {
                            self.get_next();
                            while let Some((i, c)) = self.peek_next() {
                                if c.is_digit(10) {
                                    end = *i;
                                    self.get_next();
                                } else {
                                    break;
                                }
                            }
                            let digit_end = end;
                            if let Some((_, 'f')) = self.peek_next() {
                                let mut suffix = String::with_capacity(2);
                                while let Some((i, c)) = self.peek_next() {
                                    if c.is_digit(10) {
                                        suffix.push(*c);
                                        end = *i;
                                        self.get_next();
                                    } else {
                                        break;
                                    }
                                }
                                match suffix.as_str() {
                                    "32" => Token::F32Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                    "64" => Token::F64Lit(self.raw_input[start..=digit_end].parse().unwrap()),
                                    _ => return Err(LexerError::Error {
                                        message: "Invalid float suffix".to_string(),
                                        start,
                                        end,
                                    }),
                                }
                            } else {
                                Token::FloatLit(self.raw_input[start..=end].parse().unwrap())
                            }
                        } else {
                            Token::IntLit(self.raw_input[start..=end].parse().unwrap())
                        }
                    },
                    '\'' => {
                        let mut buffer = String::new();
                        let mut count = 0;
                        while let Some((i, c)) = self.peek_next() {
                            let i = *i;
                            if *c == '\'' {
                                end = i;
                                self.get_next();
                                break;
                            } else if count > 1 && (c.is_whitespace() || *c == ':' || *c == ',' || *c == ';' || *c == ')' || *c == '}' || *c == ']')  {
                                break;
                            } else if *c == '\\' {
                                self.get_next();
                                if let Some((i, c)) = self.get_next() {
                                    end = i;
                                    match c {
                                        'n' => buffer.push('\n'),
                                        'r' => buffer.push('\r'),
                                        't' => buffer.push('\t'),
                                        '\\' => buffer.push('\\'),
                                        '\'' => buffer.push('\''),
                                        _ => return Err(LexerError::Error {
                                            message: "Invalid escape sequence".to_string(),
                                            start,
                                            end,
                                        }),
                                    }
                                } else {
                                    return Err(LexerError::Error {
                                        message: "Unexpected end of input".to_string(),
                                        start,
                                        end,
                                    });
                                }
                            } else {
                                let Some((_, c)) = self.get_next() else {
                                    panic!("Unexpected end of input after checking");
                                };
                                buffer.push(c);
                            }
                            end = i;
                            count += 1;
                        }
                        if count == 0 {
                            return Err(LexerError::Error {
                                message: "Empty character literal".to_string(),
                                start,
                                end,
                            });
                        } else if count > 1 {
                            let start = start + 1;
                            let tok = Token::Label(&self.raw_input[start..=end]);
                            tok
                        } else {
                            Token::CharLit(buffer.chars().next().unwrap())
                        }
                    },
                    '"' => {
                        let mut found_backslash = false;
                        let mut the_string = String::new();
                        while let Some((i, c)) = self.get_next() {
                            if c == '"' && !found_backslash {
                                end = i;
                                break;
                            } else if c == '\\' {
                                found_backslash = true;
                            } else if found_backslash {
                                found_backslash = false;
                                match c {
                                    'n' => the_string.push('\n'),
                                    'r' => the_string.push('\r'),
                                    't' => the_string.push('\t'),
                                    '\\' => the_string.push('\\'),
                                    '"' => the_string.push('"'),
                                    _ => return Err(LexerError::Error {
                                        message: "Invalid escape sequence".to_string(),
                                        start,
                                        end,
                                    }),
                                }
                            } else {
                                the_string.push(c);
                            }
                        }
                        Token::StringLit(the_string)
                    },
                    '{' => Token::BraceOpen,
                    '}' => Token::BraceClose,
                    '(' => Token::ParenOpen,
                    ')' => Token::ParenClose,
                    '[' => Token::BracketOpen,
                    ']' => Token::BracketClose,
                    ',' => Token::Comma,
                    ':' => {
                        if let Some((i, ':')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::Scope
                        } else {
                            Token::Colon
                        }
                    }
                    ';' => Token::Semicolon,
                    '?' => Token::Try,
                    '=' => {
                        if let Some((i, '>')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::FatArrow
                        } else if let Some((i, '=')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::Eq
                        } else {
                            Token::Assign
                        }
                    }
                    '+' => Token::Add,
                    '-' => {
                        if let Some((i, '>')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::Arrow
                        } else {
                            Token::Sub
                        }
                    }
                    '*' => Token::Mul,
                    '/' => {
                        if let Some((_, '/')) = self.peek_next() {
                            self.get_next();
                            while let Some((i, c)) = self.get_next() {
                                if c == '\n' {
                                    self.newline_pos.push(i);
                                    break;
                                }
                            }
                            return self.next_token();
                        } else if let Some((_, '*')) = self.peek_next() {
                            self.get_next();
                            while let Some((_, c)) = self.get_next() {
                                if c == '*' {
                                    if let Some((_, '/')) = self.peek_next() {
                                        self.get_next();
                                        break;
                                    }
                                }
                                if c == '\n' {
                                    self.newline_pos.push(start);
                                }
                            }
                            return self.next_token();
                        } else {
                            Token::Div
                        }
                    }
                    '%' => Token::Mod,
                    '&' => {
                        if let Some((i, '&')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::LogAnd
                        } else {
                            Token::BitAnd
                        }
                    }
                    '|' => {
                        if let Some((i, '|')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::LogOr
                        } else {
                            Token::BitOr
                        }
                    }
                    '^' => Token::BitXor,
                    '!' => {
                        if let Some((i, '=')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::Neq
                        } else {
                            Token::Not
                        }
                    },
                    '<' => {
                        if let Some((i, '=')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::Le
                        } else if let Some((i, '<')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::LShift
                        } else {
                            Token::Lt
                        }
                    },
                    '>' => {
                        if let Some((i, '=')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::Ge
                        } else if let Some((i, '=')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::Neq
                        } else if let Some((i, '-')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::InclusiveRange
                        } else if let Some((i, '>')) = self.peek_next() {
                            end = *i;
                            self.get_next();
                            Token::RShift
                        } else {
                            Token::Gt
                        }
                    },
                    '.' => {
                        if let Some((i, '.')) = self.peek_next() {
                            let i = *i;
                            self.get_next();
                            if let Some((i, '=')) = self.peek_next() {
                                end = *i;
                                self.get_next();
                                Token::InclusiveRange
                            } else {
                                end = i;
                                Token::ExclusiveRange
                            }
                        } else {
                            Token::Dot
                        }
                    },
                    '\n' => {
                        self.newline_pos.push(start);
                        return self.next_token();
                    },
                    c if c.is_whitespace() => {
                        while let Some((_, c)) = self.peek_next() {
                            if !c.is_whitespace() {
                                break;
                            }
                            self.get_next();
                        }
                        return self.next_token();
                    },
                    c => return Err(LexerError::Error {
                        message: format!("Invalid character: {}", c),
                        start,
                        end,
                    }),

                }
            }
            None => return Err(LexerError::Eof),
        };
        Ok(SpannedToken { token, start, end })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "fn main() { let x = 10; }";
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(token) => tokens.push(token),
                Err(LexerError::Eof) => break,
                Err(LexerError::Error { message, start, end }) => {
                    panic!("Error at position {}: {} at position {}", start, message, end);
                }
            }
        }
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].token, Token::Fn);
        assert_eq!(tokens[1].token, Token::Identifier("main"));
        assert_eq!(tokens[2].token, Token::ParenOpen);
        assert_eq!(tokens[3].token, Token::ParenClose);
        assert_eq!(tokens[4].token, Token::BraceOpen);
        assert_eq!(tokens[5].token, Token::Let);
        assert_eq!(tokens[6].token, Token::Identifier("x"));
        assert_eq!(tokens[7].token, Token::Assign);
        assert_eq!(tokens[8].token, Token::IntLit(10));
        assert_eq!(tokens[9].token, Token::Semicolon);
        assert_eq!(tokens[10].token, Token::BraceClose);
    }

    #[test]
    fn test_lexer_struct() {
        let input = "pub struct Test { x: u32, y: f64 }";
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(token) => tokens.push(token),
                Err(LexerError::Eof) => break,
                Err(LexerError::Error { message, start, end }) => {
                    panic!("Error at position {}: {} at position {}", start, message, end);
                }
            }
        }
        assert_eq!(tokens.len(), 12);
        assert_eq!(tokens[0].token, Token::Pub);
        assert_eq!(tokens[1].token, Token::Struct);
        assert_eq!(tokens[2].token, Token::TypeIdentifier("Test"));
        assert_eq!(tokens[3].token, Token::BraceOpen);
        assert_eq!(tokens[4].token, Token::Identifier("x"));
        assert_eq!(tokens[5].token, Token::Colon);
        assert_eq!(tokens[6].token, Token::TypeIdentifier("u32"));
        assert_eq!(tokens[7].token, Token::Comma);
        assert_eq!(tokens[8].token, Token::Identifier("y"));
        assert_eq!(tokens[9].token, Token::Colon);
        assert_eq!(tokens[10].token, Token::TypeIdentifier("f64"));
        assert_eq!(tokens[11].token, Token::BraceClose);
    }

    #[test]
    fn test_lexer_enum() {
        let input = "enum Test { A, B, C }";
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(token) => tokens.push(token),
                Err(LexerError::Eof) => break,
                Err(LexerError::Error { message, start, end }) => {
                    panic!("Error at position {}: {} at position {}", start, message, end);
                }
            }
        }
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].token, Token::Enum);
        assert_eq!(tokens[1].token, Token::TypeIdentifier("Test"));
        assert_eq!(tokens[2].token, Token::BraceOpen);
        assert_eq!(tokens[3].token, Token::TypeIdentifier("A"));
        assert_eq!(tokens[4].token, Token::Comma);
        assert_eq!(tokens[5].token, Token::TypeIdentifier("B"));
        assert_eq!(tokens[6].token, Token::Comma);
        assert_eq!(tokens[7].token, Token::TypeIdentifier("C"));
        assert_eq!(tokens[8].token, Token::BraceClose);
    }

    #[test]
    fn test_lexer_trait() {
        let input = "trait Test { fn test(self); }";
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(token) => tokens.push(token),
                Err(LexerError::Eof) => break,
                Err(LexerError::Error { message, start, end }) => {
                    panic!("Error at position {}: {} at position {}", start, message, end);
                }
            }
        }
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].token, Token::Trait);
        assert_eq!(tokens[1].token, Token::TypeIdentifier("Test"));
        assert_eq!(tokens[2].token, Token::BraceOpen);
        assert_eq!(tokens[3].token, Token::Fn);
        assert_eq!(tokens[4].token, Token::Identifier("test"));
        assert_eq!(tokens[5].token, Token::ParenOpen);
        assert_eq!(tokens[6].token, Token::Self_);
        assert_eq!(tokens[7].token, Token::ParenClose);
        assert_eq!(tokens[8].token, Token::Semicolon);
        assert_eq!(tokens[9].token, Token::BraceClose);
    }

    #[test]
    fn test_lexer_comments() {
        let input = "fn main() { // This is a comment\n let x = 10; }";
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(token) => tokens.push(token),
                Err(LexerError::Eof) => break,
                Err(LexerError::Error { message, start, end }) => {
                    panic!("Error at position {}: {} at position {}", start, message, end);
                }
            }
        }
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].token, Token::Fn);
        assert_eq!(tokens[1].token, Token::Identifier("main"));
        assert_eq!(tokens[2].token, Token::ParenOpen);
        assert_eq!(tokens[3].token, Token::ParenClose);
        assert_eq!(tokens[4].token, Token::BraceOpen);
        assert_eq!(tokens[5].token, Token::Let);
        assert_eq!(tokens[6].token, Token::Identifier("x"));
        assert_eq!(tokens[7].token, Token::Assign);
        assert_eq!(tokens[8].token, Token::IntLit(10));
        assert_eq!(tokens[9].token, Token::Semicolon);
        assert_eq!(tokens[10].token, Token::BraceClose);
    }

    #[test]
    fn test_lexer_multiline_comments() {
        let input = "fn main() { /* This is a\nmultiline comment */ let x = 10; }";
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(token) => tokens.push(token),
                Err(LexerError::Eof) => break,
                Err(LexerError::Error { message, start, end }) => {
                    panic!("Error at position {}: {} at position {}", start, message, end);
                }
            }
        }
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].token, Token::Fn);
        assert_eq!(tokens[1].token, Token::Identifier("main"));
        assert_eq!(tokens[2].token, Token::ParenOpen);
        assert_eq!(tokens[3].token, Token::ParenClose);
        assert_eq!(tokens[4].token, Token::BraceOpen);
        assert_eq!(tokens[5].token, Token::Let);
        assert_eq!(tokens[6].token, Token::Identifier("x"));
        assert_eq!(tokens[7].token, Token::Assign);
        assert_eq!(tokens[8].token, Token::IntLit(10));
        assert_eq!(tokens[9].token, Token::Semicolon);
        assert_eq!(tokens[10].token, Token::BraceClose);
    }

    #[test]
    fn test_trait_impl() {
        let input = "impl Test for Test { fn test(self); }";
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(token) => tokens.push(token),
                Err(LexerError::Eof) => break,
                Err(LexerError::Error { message, start, end }) => {
                    panic!("Error at position {}: {} at position {}", start, message, end);
                }
            }
        }
        assert_eq!(tokens.len(), 12);
        assert_eq!(tokens[0].token, Token::Impl);
        assert_eq!(tokens[1].token, Token::TypeIdentifier("Test"));
        assert_eq!(tokens[2].token, Token::For);
        assert_eq!(tokens[3].token, Token::TypeIdentifier("Test"));
        assert_eq!(tokens[4].token, Token::BraceOpen);
        assert_eq!(tokens[5].token, Token::Fn);
        assert_eq!(tokens[6].token, Token::Identifier("test"));
        assert_eq!(tokens[7].token, Token::ParenOpen);
        assert_eq!(tokens[8].token, Token::Self_);
        assert_eq!(tokens[9].token, Token::ParenClose);
        assert_eq!(tokens[10].token, Token::Semicolon);
        assert_eq!(tokens[11].token, Token::BraceClose);
    }
}
