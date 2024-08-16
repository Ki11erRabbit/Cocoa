use crate::lexer::{Lexer, SpannedToken, LexerError};
use crate::parser::ParserError;
use crate::parser::ParseResult;




pub struct ParseTable<'a> {
    lexer: Lexer<'a>
}

impl<'a> ParseTable<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        ParseTable {
            lexer,
        }
    }

    pub fn next(&mut self) -> ParseResult<SpannedToken> {
        let next = self.lexer.next();
        match next {
            Some(token) => {
                match token {
                    Ok(token) => {
                        Ok(token)
                    }
                    Err(LexerError::Error{ message, start, end}) => Err(ParserError::new(&message, start, end)),
                    Err(LexerError::Eof) => Err(ParserError::EOF),
                }
            }
            None => Err(ParserError::EOF),
        }
    }

    pub fn peek(&mut self) -> ParseResult<&SpannedToken> {
        let next = self.lexer.peek();
        match next {
            Some(token) => {
                match token {
                    Ok(token) => {
                        Ok(token)
                    }
                    Err(LexerError::Error{ message, start, end}) => Err(ParserError::new(&message, *start, *end)),
                    Err(LexerError::Eof) => Err(ParserError::EOF),
                }
            }
            None => Err(ParserError::EOF),
        }
    }
}

