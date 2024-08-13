use std::str::CharIndices;

pub trait LexerIterator<'a> {
    fn next_token(&mut self) -> LexerResult<'a>;
}

pub type LexerResult<'a> = Result<SpannedToken<'a>, LexerError>;

pub enum LexerError {
    Error {
        message: String,
        column: usize,
        line: usize,
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
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            raw_input: input,
            input: input.char_indices().peekable(),
            newline_pos: vec![0],
        }
    }

    pub fn get_current_line(&self) -> usize {
        self.newline_pos.last().unwrap().clone() + 1
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}


impl<'a> LexerIterator<'a> for Lexer<'a> {

    fn next_token(&mut self) -> LexerResult<'a> {
        let mut end;
        let start;
        let token = match self.input.next() {
            Some((startt, c)) => {
                start = startt;
                end = start;
                match c {
                    'a'..='z' | '_' => {
                        while let Some((i, c)) = self.input.peek() {
                            if c.is_alphanumeric() || *c == '_' {
                                end = *i;
                                self.input.next();
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
                        while let Some((i, c)) = self.input.peek() {
                            if c.is_alphanumeric() || *c == '_' {
                                end = *i;
                                self.input.next();
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
                        while let Some((i, c)) = self.input.peek() {
                            if c.is_digit(10) {
                                end = *i;
                                self.input.next();
                            } else {
                                break;
                            }
                        }
                        if let Some((_, 'u')) = self.input.peek() {
                            self.input.next();
                            let mut suffix = String::with_capacity(2);
                            while let Some((i, c)) = self.input.peek() {
                                if c.is_digit(10) {
                                    suffix.push(*c);
                                    end = *i;
                                    self.input.next();
                                } else {
                                    break;
                                }
                            }
                            match suffix.as_str() {
                                "8" => Token::U8Lit(self.raw_input[start..=end].parse().unwrap()),
                                "16" => Token::U16Lit(self.raw_input[start..=end].parse().unwrap()),
                                "32" => Token::U32Lit(self.raw_input[start..=end].parse().unwrap()),
                                "64" => Token::U64Lit(self.raw_input[start..=end].parse().unwrap()),
                                _ => return Err(LexerError::Error {
                                    message: "Invalid unsigned integer suffix".to_string(),
                                    column: start,
                                    line: self.get_current_line(),
                                }),
                            }
                            
                        } else if let Some((_, 'i')) = self.input.peek() {
                            self.input.next();
                            let mut suffix = String::with_capacity(2);
                            while let Some((i, c)) = self.input.peek() {
                                if c.is_digit(10) {
                                    suffix.push(*c);
                                    end = *i;
                                    self.input.next();
                                } else {
                                    break;
                                }
                            }
                            match suffix.as_str() {
                                "8" => Token::I8Lit(self.raw_input[start..=end].parse().unwrap()),
                                "16" => Token::I16Lit(self.raw_input[start..=end].parse().unwrap()),
                                "32" => Token::I32Lit(self.raw_input[start..=end].parse().unwrap()),
                                "64" => Token::I64Lit(self.raw_input[start..=end].parse().unwrap()),
                                _ => return Err(LexerError::Error {
                                    message: "Invalid signed integer suffix".to_string(),
                                    column: start,
                                    line: self.get_current_line(),
                                }),
                            }
                        } else if let Some((_, '.')) = self.input.peek() {
                            self.input.next();
                            while let Some((i, c)) = self.input.peek() {
                                if c.is_digit(10) {
                                    end = *i;
                                    self.input.next();
                                } else {
                                    break;
                                }
                            }
                            if let Some((_, 'f')) = self.input.peek() {
                                let mut suffix = String::with_capacity(2);
                                while let Some((i, c)) = self.input.peek() {
                                    if c.is_digit(10) {
                                        suffix.push(*c);
                                        end = *i;
                                        self.input.next();
                                    } else {
                                        break;
                                    }
                                }
                                match suffix.as_str() {
                                    "32" => Token::F32Lit(self.raw_input[start..=end].parse().unwrap()),
                                    "64" => Token::F64Lit(self.raw_input[start..=end].parse().unwrap()),
                                    _ => return Err(LexerError::Error {
                                        message: "Invalid float suffix".to_string(),
                                        column: start,
                                        line: self.get_current_line(),
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
                        let mut size = 0;
                        let mut found_backslash = false;
                        let mut the_char = None;
                        let mut the_label = None;
                        let mut found_closing_quote = false;
                        while let Some((i, c)) = self.input.next() {
                            if c == '\'' && !found_backslash && size == 1 {
                                end = i;
                                found_closing_quote = true;
                                break;
                            } else if !c.is_alphanumeric() && size > 1 {
                                the_label = Some(Token::Label(&self.raw_input[start..=end]));
                                the_char = None;
                                break;
                            } else if c == '\\' && size == 0 {
                                found_backslash = true;
                            } else if found_backslash && size == 0 {
                                found_backslash = false;
                                match c {
                                    'n' => the_char = Some('\n'),
                                    'r' => the_char = Some('\r'),
                                    't' => the_char = Some('\t'),
                                    '\\' => the_char = Some('\\'),
                                    '\'' => the_char = Some('\''),
                                    _ => return Err(LexerError::Error {
                                        message: "Invalid escape sequence".to_string(),
                                        column: i,
                                        line: self.get_current_line(),
                                    }),
                                }
                            } else {
                                size += 1;
                                the_char = Some(c);
                            }
                        }
                        match (the_char, the_label) {
                            (Some(c), None) => Token::CharLit(c),
                            (None, Some(tok)) => tok,
                            (None, None) if found_closing_quote => return Err(LexerError::Error {
                                message: "Invalid character literal".to_string(),
                                column: start,
                                line: self.get_current_line(),
                            }),
                            _ => return Err(LexerError::Error {
                                message: "Invalid Label".to_string(),
                                column: start,
                                line: self.get_current_line(),
                            }),
                        }
                    },
                    '"' => {
                        let mut found_backslash = false;
                        let mut the_string = String::new();
                        while let Some((i, c)) = self.input.next() {
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
                                        column: i,
                                        line: self.get_current_line(),
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
                        if let Some((i, ':')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::Scope
                        } else {
                            Token::Colon
                        }
                    }
                    ';' => Token::Semicolon,
                    '?' => Token::Try,
                    '=' => {
                        if let Some((i, '>')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::FatArrow
                        } else if let Some((i, '=')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::Eq
                        } else {
                            Token::Assign
                        }
                    }
                    '+' => Token::Add,
                    '-' => {
                        if let Some((i, '>')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::Arrow
                        } else {
                            Token::Sub
                        }
                    }
                    '*' => Token::Mul,
                    '/' => {
                        if let Some((_, '/')) = self.input.peek() {
                            self.input.next();
                            while let Some((i, c)) = self.input.next() {
                                if c == '\n' {
                                    self.newline_pos.push(i);
                                    break;
                                }
                            }
                            return self.next_token();
                        } else if let Some((_, '*')) = self.input.peek() {
                            self.input.next();
                            while let Some((_, c)) = self.input.next() {
                                if c == '*' {
                                    if let Some((_, '/')) = self.input.peek() {
                                        self.input.next();
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
                        if let Some((i, '&')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::LogAnd
                        } else {
                            Token::BitAnd
                        }
                    }
                    '|' => {
                        if let Some((i, '|')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::LogOr
                        } else {
                            Token::BitOr
                        }
                    }
                    '^' => Token::BitXor,
                    '!' => Token::Not,
                    '<' => {
                        if let Some((i, '=')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::Le
                        } else if let Some((i, '<')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::LShift
                        } else {
                            Token::Lt
                        }
                    },
                    '>' => {
                        if let Some((i, '=')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::Ge
                        } else if let Some((i, '=')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::Neq
                        } else if let Some((i, '-')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::InclusiveRange
                        } else if let Some((i, '>')) = self.input.peek() {
                            end = *i;
                            self.input.next();
                            Token::RShift
                        } else {
                            Token::Gt
                        }
                    },
                    '.' => {
                        if let Some((i, '.')) = self.input.peek() {
                            let i = *i;
                            self.input.next();
                            if let Some((i, '=')) = self.input.peek() {
                                end = *i;
                                self.input.next();
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
                        while let Some((_, c)) = self.input.peek() {
                            if !c.is_whitespace() {
                                break;
                            }
                            self.input.next();
                        }
                        return self.next_token();
                    },
                    c => return Err(LexerError::Error {
                        message: format!("Invalid character: {}", c),
                        column: start,
                        line: self.get_current_line(),
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
                Err(LexerError::Error { message, column, line }) => {
                    panic!("Error at line {}: {} at column {}", line, message, column);
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
                Err(LexerError::Error { message, column, line }) => {
                    panic!("Error at line {}: {} at column {}", line, message, column);
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
                Err(LexerError::Error { message, column, line }) => {
                    panic!("Error at line {}: {} at column {}", line, message, column);
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
                Err(LexerError::Error { message, column, line }) => {
                    panic!("Error at line {}: {} at column {}", line, message, column);
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
                Err(LexerError::Error { message, column, line }) => {
                    panic!("Error at line {}: {} at column {}", line, message, column);
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
                Err(LexerError::Error { message, column, line }) => {
                    panic!("Error at line {}: {} at column {}", line, message, column);
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
                Err(LexerError::Error { message, column, line }) => {
                    panic!("Error at line {}: {} at column {}", line, message, column);
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
