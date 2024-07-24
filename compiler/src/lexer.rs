use std::str::CharIndices;

use crate::token::Token;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;


pub enum LexicalError {

}

pub struct Lexer<'a> {
    input: &'a str,
    chars: CharIndices<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.char_indices();
        Lexer {
            input,
            chars,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, ' ')) => continue,
                Some((_, '\n')) => continue,
                Some((_, '\r')) => continue,
                Some((i, '{')) => return Some(Ok((i, Token::LeftBrace, i + 1))),
                Some((i, '}')) => return Some(Ok((i, Token::RightBrace, i + 1))),
                Some((i, '(')) => return Some(Ok((i, Token::LeftParen, i + 1))),
                Some((i, ')')) => return Some(Ok((i, Token::RightParen, i + 1))),
                Some((i, '[')) => return Some(Ok((i, Token::LeftBracket, i + 1))),
                Some((i, ']')) => return Some(Ok((i, Token::RightBracket, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Token::Comma, i + 1))),
                Some((i, '.')) => {
                    match self.chars.clone().next() {
                        Some((_, '.')) => {
                            self.chars.next();
                            match self.chars.clone().next() {
                                Some((_, '=')) => {
                                    self.chars.next();
                                    return Some(Ok((i, Token::ExclusiveRange, i + 3)));
                                }
                                _ => return Some(Ok((i, Token::InclusiveRange, i + 2))),
                            }
                        }
                        _ => return Some(Ok((i, Token::Dot, i + 1))),
                    }
                }
                Some((i, ';')) => return Some(Ok((i, Token::Semicolon, i + 1))),
                Some((i, ':')) => return Some(Ok((i, Token::Colon, i + 1))),
                Some((i, '+')) => return Some(Ok((i, Token::Plus, i + 1))),
                Some((i, '-')) => return Some(Ok((i, Token::Minus, i + 1))),
                Some((i, '*')) => return Some(Ok((i, Token::Multiply, i + 1))),
                Some((i, '\'')) => {
                    match self.chars.clone().next() {
                        Some((_, c)) => {
                            self.chars.next();
                            match self.chars.clone().next() {
                                Some((_, '\'')) => {
                                    self.chars.next();
                                    return Some(Ok((i, Token::Char(c), i + 2)));
                                }
                                _ => {}
                            }
                        }
                        None => {}
                    }
                }
                Some((i, '"')) => {
                    let start = i;
                    let mut found_backslash = false;
                    while let Some((_, c)) = self.chars.clone().next() {
                        match c {
                            '"' => {
                                self.chars.next();
                                if !found_backslash {
                                    break;
                                }
                            }
                            '\\' => {
                                found_backslash = true;
                                self.chars.next();
                            }
                            _ => {
                                self.chars.next();
                            }
                        }
                    }
                    let end = self.chars.as_str().char_indices().next().unwrap().0;
                    let string = &self.input[start + 1..end - 1];
                    return Some(Ok((start, Token::String(string.to_string()), end)));
                }
                Some((i, '/')) => {
                    match self.chars.clone().next() {
                        Some((_, '/')) => {
                            while let Some((_, c)) = self.chars.clone().next() {
                                match c {
                                    '\n' => break,
                                    _ => {
                                        self.chars.next();
                                    }
                                }
                            }
                            continue;
                            //return Some(Ok((i, Token::Comment, i + 1)));
                        }
                        Some((_, '*')) => {
                            self.chars.next();
                            while let Some((_, c)) = self.chars.clone().next() {
                                match c {
                                    '*' => {
                                        self.chars.next();
                                        match self.chars.clone().next() {
                                            Some((_, '/')) => {
                                                self.chars.next();
                                                break;
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {
                                        self.chars.next();
                                    }
                                }
                            }
                            continue;
                            //return Some(Ok((i, Token::Comment, i + 1)));
                        }
                        _ => return Some(Ok((i, Token::Divide, i + 1))),
                    }
                }
                Some((i, '%')) => return Some(Ok((i, Token::Modulo, i + 1))),
                Some((i, '=')) => {
                    match self.chars.clone().next() {
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((i, Token::Equal, i + 2)));
                        }
                        _ => return Some(Ok((i, Token::Assign, i + 1))),
                    }
                }
                Some((i, '!')) => {
                    match self.chars.clone().next() {
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((i, Token::NotEqual, i + 2)));
                        }
                        _ => return Some(Ok((i, Token::Negate, i + 1))),
                    }
                }
                Some((i, '<')) => {
                    match self.chars.clone().next() {
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((i, Token::LessThanOrEqual, i + 2)));
                        }
                        Some((_, '<')) => {
                            self.chars.next();
                            return Some(Ok((i, Token::BitwiseShiftLeft, i + 2)));
                        }
                        _ => return Some(Ok((i, Token::LessThan, i + 1))),
                    }
                }
                Some((i, '>')) => {
                    match self.chars.clone().next() {
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((i, Token::GreaterThanOrEqual, i + 2)));
                        }
                        Some((_, '>')) => {
                            self.chars.next();
                            return Some(Ok((i, Token::BitwiseShiftRight, i + 2)));
                        }
                        _ => return Some(Ok((i, Token::GreaterThan, i + 1))),
                    }
                }
                Some((i, '&')) => {
                    match self.chars.clone().next() {
                        Some((_, '&')) => {
                            self.chars.next();
                            return Some(Ok((i, Token::And, i + 2)));
                        }
                        _ => return Some(Ok((i, Token::BitwiseAnd, i + 1))),
                    }
                }
                Some((i, '|')) => {
                    match self.chars.clone().next() {
                        Some((_, '|')) => {
                            self.chars.next();
                            return Some(Ok((i, Token::Or, i + 2)));
                        }
                        _ => return Some(Ok((i, Token::BitwiseOr, i + 1))),
                    }
                }
                Some((i, '^')) => return Some(Ok((i, Token::BitwiseXor, i + 1))),
                Some((i, '@')) => return Some(Ok((i, Token::At, i + 1))),
                Some((i, c)) => {
                    match c {
                        'a'..='z' | 'A'..='Z' | '_' => {
                            let start = i;
                            while let Some((_, c)) = self.chars.clone().next() {
                                match c {
                                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                                        self.chars.next();
                                    }
                                    _ => break,
                                }
                            }
                            let end = self.chars.as_str().char_indices().next().unwrap().0;
                            let ident = &self.input[start..end];

                            match ident {
                                "let" => return Some(Ok((start, Token::Let, end))),
                                "class" => return Some(Ok((start, Token::Class, end))),
                                "interface" => return Some(Ok((start, Token::Interface, end))),
                                "abstract" => return Some(Ok((start, Token::Abstract, end))),
                                "pub" => return Some(Ok((start, Token::Public, end))),
                                "prot" => return Some(Ok((start, Token::Protected, end))),
                                "implements" => return Some(Ok((start, Token::Implements, end))),
                                "extends" => return Some(Ok((start, Token::Extends, end))),
                                "true" => return Some(Ok((start, Token::Bool(true), end))),
                                "false" => return Some(Ok((start, Token::Bool(false), end))),
                                "const" => return Some(Ok((start, Token::Const, end))),
                                "static" => return Some(Ok((start, Token::Static, end))),
                                "if" => return Some(Ok((start, Token::If, end))),
                                "else" => return Some(Ok((start, Token::Else, end))),
                                "while" => return Some(Ok((start, Token::While, end))),
                                "for" => return Some(Ok((start, Token::For, end))),
                                "in" => return Some(Ok((start, Token::In, end))),
                                "break" => return Some(Ok((start, Token::Break, end))),
                                "continue" => return Some(Ok((start, Token::Continue, end))),
                                "return" => return Some(Ok((start, Token::Return, end))),
                                "loop" => return Some(Ok((start, Token::Loop, end))),
                                "fn" => return Some(Ok((start, Token::Fn, end))),
                                "as" => return Some(Ok((start, Token::As, end))),
                                "null" => return Some(Ok((start, Token::Null, end))),
                                "new" => return Some(Ok((start, Token::New, end))),
                                "package" => return Some(Ok((start, Token::Package, end))),
                                "import" => return Some(Ok((start, Token::Import, end))),
                                "instanceof" => return Some(Ok((start, Token::InstanceOf, end))),
                                "u8" => return Some(Ok((start, Token::U8, end))),
                                "u16" => return Some(Ok((start, Token::U16, end))),
                                "u32" => return Some(Ok((start, Token::U32, end))),
                                "u64" => return Some(Ok((start, Token::U64, end))),
                                "i8" => return Some(Ok((start, Token::I8, end))),
                                "i16" => return Some(Ok((start, Token::I16, end))),
                                "i32" => return Some(Ok((start, Token::I32, end))),
                                "i64" => return Some(Ok((start, Token::I64, end))),
                                "f32" => return Some(Ok((start, Token::F32, end))),
                                "f64" => return Some(Ok((start, Token::F64, end))),
                                "boolean" => return Some(Ok((start, Token::BoolType, end))),
                                "char" => return Some(Ok((start, Token::CharType, end))),
                                _ => {}
                            }
                            
                            return Some(Ok((start, Token::Identifier(ident.to_string()), end)));
                        }
                        '0'..='9' => {
                            let start = i;
                            while let Some((_, c)) = self.chars.clone().next() {
                                match c {
                                    '0'..='9' => {
                                        self.chars.next();
                                    }
                                    _ => break,
                                }
                            }
                            let end = self.chars.as_str().char_indices().next().unwrap().0;
                            let num = &self.input[start..end];
                            return Some(Ok((start, Token::Integer(num.to_string()), end)));
                        }
                        _ => {}
                    }
                }
                None => return None,
            }

        }
    }

}
