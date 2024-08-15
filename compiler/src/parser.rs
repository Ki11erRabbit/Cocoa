use crate::{ast::{BinaryOperator, Expression, Literal, Pattern, PrefixOperator, SpannedExpression, SpannedPattern, SpannedStatement, SpannedType, Statement, Type}, lexer::{Lexer, LexerError, SpannedToken, Token}};

pub type ParseResult<'a, T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    Error {
        message: String,
        column: usize,
        line: usize,
    },
    EOF,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}


impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        let lexer = Lexer::new(input);
        Parser {
            lexer,
        }
    }

    pub fn next(&mut self) -> ParseResult<SpannedToken> {
        let next = self.lexer.next();
        match next {
            Some(token) => {
                match token {
                    Ok(token) => Ok(token),
                    Err(LexerError::Error{ message, column, line}) => Err(ParserError::Error {
                        message,
                        column,
                        line,
                    }),
                    Err(LexerError::Eof) => Err(ParserError::EOF),
                }
            }
            None => Err(ParserError::EOF),
        }
    }

    pub fn peek(&mut self) -> ParseResult<&SpannedToken> {
        match self.lexer.peek() {
            Some(token) => {
                match token {
                    Ok(token) => Ok(token),
                    Err(LexerError::Error{ message, column, line}) => Err(ParserError::Error {
                        message: message.clone(),
                        column: *column,
                        line: *line,
                    }),
                    Err(LexerError::Eof) => Err(ParserError::EOF),
                }
            }
            None => Err(ParserError::EOF),
        }
    }

    pub fn parse_block_body(&mut self) -> ParseResult<Vec<SpannedStatement>> {
        let mut statements = Vec::new();
        let mut found_eof = false;
        while !found_eof {
            match self.parse_statement() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(ParserError::EOF) => {
                    found_eof = true;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(statements)
    }
    

    pub fn parse_statement(&mut self) -> ParseResult<SpannedStatement> {
        let res = self.parse_expression_for_statement();
        match res {
            None => {},
            Some(expr) => {
                match self.peek() {
                    Ok(SpannedToken { token: Token::Semicolon, .. }) => {
                        let SpannedToken { end, .. } = self.next()?;
                        let expr = expr?;
                        let start = expr.start;
                        return Ok(SpannedStatement {
                            statement: Statement::Expression(expr),
                            start,
                            end,
                        });
                    }
                    _ => {
                        let expr = expr?;
                        let start = expr.start;
                        let end = expr.end;
                        return Ok(SpannedStatement {
                        statement: Statement::HangingExpression(expr),
                        start,
                        end,
                        })
                    },
                }
            }
        }
        if let Ok(SpannedToken { token: Token::Let, start, .. }) = self.peek() {
            let start = *start;
            return self.parse_let_statement(start);
        }

        todo!("Implement remaining statements")
    }

    fn parse_let_statement(&mut self, start: usize) -> ParseResult<SpannedStatement> {
        let Ok(SpannedToken { token: Token::Let, start, .. }) = self.next() else {
            let (line, column) = self.lexer.get_coord_from_pos(start);
            return Err(ParserError::Error {
                message: "Expected let keyword".to_string(),
                column,
                line,
            });
        };
        let pattern = self.parse_pattern()?;
        let ty = self.parse_type()?;
        let Ok(SpannedToken { token: Token::Assign, start, .. }) = self.next() else {
            let (line, column) = self.lexer.get_coord_from_pos(start);
            return Err(ParserError::Error {
                message: "Expected assignment operator".to_string(),
                column,
                line,
            });
        };
        let expression = self.parse_expression()?;
        let Ok(SpannedToken { token: Token::Semicolon, end, start }) = self.next() else {
            let (line, column) = self.lexer.get_coord_from_pos(start);
            return Err(ParserError::Error {
                message: "Expected semicolon".to_string(),
                column,
                line,
            });
        };
        Ok(SpannedStatement {
            statement: Statement::LetStatement {
                binding: pattern,
                type_annotation: ty,
                expression,
            },
            start,
            end,
        })
    }

    fn parse_pattern(&mut self) -> ParseResult<SpannedPattern> {
        if let Ok(SpannedToken { token: Token::Identifier(_), .. }) = self.peek() {
            let SpannedToken { token: Token::Identifier(id), start, end } = self.next()? else {
                panic!("Expected identifier after checking that it is an identifier");
            };
            return Ok(SpannedPattern {
                pattern: Pattern::Identifier(id.to_string()),
                start,
                end,
            })
        }
        todo!("Implement remaining patterns")
    }

    fn parse_expression_for_statement(&mut self) -> Option<ParseResult<SpannedExpression>> {
        match self.peek() {
            Ok(SpannedToken { token: Token::Let, .. }) |
            Ok(SpannedToken { token: Token::Semicolon, .. }) |
            Ok(SpannedToken { token: Token::Arrow, .. }) |
            Ok(SpannedToken { token: Token::FatArrow, .. }) |
            Ok(SpannedToken { token: Token::BraceClose, .. }) => {
                return None
            }
            _ => {}
        }

        Some(self.parse_expression())
    }

    fn parse_expression(&mut self) -> ParseResult<SpannedExpression> {

        // TODO parse if expression
        // TODO parse return expression
        // TODO parse break expression
        // TODO parse closure expression
        self.parse_range_expression()
    }

    fn parse_range_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse range expression

        self.parse_disjunction_expression()
    }

    fn parse_disjunction_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse disjunction expression

        self.parse_conjunction_expression()
    }

    fn parse_conjunction_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse conjunction expression

        self.parse_equality_and_order_expression()
    }

    fn parse_equality_and_order_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse equality and order expression

        self.parse_bitwise_or()
    }

    fn parse_bitwise_or(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse bitwise or expression

        self.parse_bitwise_xor()
    }

    fn parse_bitwise_xor(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse bitwise xor expression

        self.parse_bitwise_and()
    }

    fn parse_bitwise_and(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse bitwise and expression

        self.parse_shift_expression()
    }

    fn parse_shift_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse shift expression

        self.parse_addition_expression()
    }

    fn parse_addition_expression(&mut self) -> ParseResult<SpannedExpression> {
        let expr1 = self.parse_multiplication_expression()?;
        let op = match self.peek() {
            Ok(SpannedToken { token: Token::Add, .. }) => {
                self.next()?;
                BinaryOperator::Add
            }
            Ok(SpannedToken { token: Token::Sub, .. }) => {
                self.next()?;
                BinaryOperator::Subtract
            }
            _ => return Ok(expr1),
        };

        let expr2 = self.parse_addition_expression()?;

        let start = expr1.start;
        let end = expr2.end;

        Ok(SpannedExpression {
            expression: Expression::BinaryExpression {
                left: Box::new(expr1),
                operator: op,
                right: Box::new(expr2),
            },
            start,
            end,
        })
    }

    fn parse_multiplication_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse multiplication expression
        let expr1 = self.parse_unary_expression()?;

        let op = match self.peek() {
            Ok(SpannedToken { token: Token::Mul, .. }) => {
                self.next()?;
                BinaryOperator::Multiply
            }
            Ok(SpannedToken { token: Token::Div, .. }) => {
                self.next()?;
                BinaryOperator::Divide
            }
            Ok(SpannedToken { token: Token::Mod, .. }) => {
                self.next()?;
                BinaryOperator::Modulo
            }
            _ => return Ok(expr1),
        };

        let expr2 = self.parse_multiplication_expression()?;
        let start = expr1.start;
        let end = expr2.end;

        Ok(SpannedExpression {
            expression: Expression::BinaryExpression {
                left: Box::new(expr1),
                operator: op,
                right: Box::new(expr2),
            },
            start,
            end,
        })
    }

    fn parse_unary_expression(&mut self) -> ParseResult<SpannedExpression> {
        match self.peek() {
            Ok(SpannedToken { token: Token::Sub, ..}) => {
                let SpannedToken { start, .. } = self.next()?;
                let expression = self.parse_expression()?;
                let end = expression.end;
                let expression = Expression::PrefixExpression {
                    operator: PrefixOperator::Negate,
                    right: Box::new(expression)
                };
                Ok(SpannedExpression {
                    expression,
                    start,
                    end,
                })
            }
            Ok(SpannedToken { token: Token::Not, ..}) => {
                let SpannedToken { start, .. } = self.next()?;
                let expression = self.parse_expression()?;
                let end = expression.end;
                let expression = Expression::PrefixExpression {
                    operator: PrefixOperator::Not,
                    right: Box::new(expression)
                };
                Ok(SpannedExpression {
                    expression,
                    start,
                    end,
                })
            }
            _ => self.parse_try_expression(),
        }
    }

    fn parse_try_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse try expression

        self.parse_primary_expression()
    }

    fn parse_primary_expression(&mut self) -> ParseResult<SpannedExpression> {
        match self.peek()? {
            SpannedToken { token: Token::U8Lit(_), .. } => {
                let SpannedToken { token: Token::U8Lit(lit), start, end } = self.next()? else {
                    panic!("Expected u8 literal after checking that it is a u8 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::U8(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::I8Lit(_), .. } => {
                let SpannedToken { token: Token::I8Lit(lit), start, end } = self.next()? else {
                    panic!("Expected i8 literal after checking that it is a i8 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::I8(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::U16Lit(_), .. } => {
                let SpannedToken { token: Token::U16Lit(lit), start, end } = self.next()? else {
                    panic!("Expected u16 literal after checking that it is a u16 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::U16(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::I16Lit(_), .. } => {
                let SpannedToken { token: Token::I16Lit(lit), start, end } = self.next()? else {
                    panic!("Expected i16 literal after checking that it is a i16 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::I16(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::U32Lit(_), .. } => {
                let SpannedToken { token: Token::U32Lit(lit), start, end } = self.next()? else {
                    panic!("Expected u32 literal after checking that it is a u32 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::U32(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::I32Lit(_), .. } => {
                let SpannedToken { token: Token::I32Lit(lit), start, end } = self.next()? else {
                    panic!("Expected i32 literal after checking that it is a i32 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::I32(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::U64Lit(_), .. } => {
                let SpannedToken { token: Token::U64Lit(lit), start, end } = self.next()? else {
                    panic!("Expected u64 literal after checking that it is a u64 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::U64(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::I64Lit(_), .. } => {
                let SpannedToken { token: Token::I64Lit(lit), start, end } = self.next()? else {
                    panic!("Expected i64 literal after checking that it is a i64 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::I64(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::IntLit(_), .. } => {
                let SpannedToken { token: Token::IntLit(lit), start, end } = self.next()? else {
                    panic!("Expected int literal after checking that it is a int literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::F32Lit(_), .. } => {
                let SpannedToken { token: Token::F32Lit(lit), start, end } = self.next()? else {
                    panic!("Expected f32 literal after checking that it is a f32 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::F32(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::F64Lit(_), .. } => {
                let SpannedToken { token: Token::F64Lit(lit), start, end } = self.next()? else {
                    panic!("Expected f64 literal after checking that it is a f64 literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::F64(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::FloatLit(_), .. } => {
                let SpannedToken { token: Token::FloatLit(lit), start, end } = self.next()? else {
                    panic!("Expected float literal after checking that it is a float literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::F64(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::CharLit(_), .. } => {
                let SpannedToken { token: Token::CharLit(lit), start, end } = self.next()? else {
                    panic!("Expected char literal after checking that it is a char literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::Char(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::StringLit(_), .. } => {
                let SpannedToken { token: Token::StringLit(lit), start, end } = self.next()? else {
                    panic!("Expected string literal after checking that it is a string literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::String(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::BoolLit(_), .. } => {
                let SpannedToken { token: Token::BoolLit(lit), start, end } = self.next()? else {
                    panic!("Expected bool literal after checking that it is a bool literal");
                };
                Ok(SpannedExpression {
                    expression: Expression::Literal(Literal::Bool(lit)),
                    start,
                    end,
                })
            }
            SpannedToken { token: Token::Identifier(_), .. } => {
                let SpannedToken { token: Token::Identifier(id), start, end } = self.next()? else {
                    panic!("Expected identifier after checking that it is an identifier");
                };
                Ok(SpannedExpression {
                    expression: Expression::Variable(id.to_string()),
                    start,
                    end
                })
            }
            SpannedToken { token, start, .. } => {
                println!("{:?}", token);
                let start = *start;
                drop(token);
                let (line, column) = self.lexer.get_coord_from_pos(start);
                Err(ParserError::Error {
                    message: "Expected primary expression".to_string(),
                    column,
                    line,
                })
            }
        }
    }
}

// Type Parsing
impl<'a> Parser<'a> {
    fn parse_type(&mut self) -> ParseResult<Option<SpannedType>> {
        let Ok(SpannedToken { token: Token::Colon, .. }) = self.peek() else {
            return Ok(None);
        };
        let SpannedToken { end, .. } = self.next()?;
        let Ok(SpannedToken { token: _, start, end: _ }) = self.peek() else {
            let (line, column) = self.lexer.get_coord_from_pos(end);
            return Err(ParserError::Error {
                message: "Expected type identifier".to_string(),
                column,
                line,
            });
        };
        let start = *start;
        let Ok(SpannedToken { token: Token::TypeIdentifier(id), start, end }) = self.next() else {
            let (line, column) = self.lexer.get_coord_from_pos(start);
            return Err(ParserError::Error {
                message: "Expected type identifier".to_string(),
                column,
                line,
            });
        };
        match id {
            "u8" => Ok(Some(SpannedType {
                type_: Type::U8,
                start,
                end,
            })),
            "u16" => Ok(Some(SpannedType {
                type_: Type::U16,
                start,
                end,
            })),
            "u32" => Ok(Some(SpannedType {
                type_: Type::U32,
                start,
                end,
            })),
            "u64" => Ok(Some(SpannedType {
                type_: Type::U64,
                start,
                end,
            })),
            "i8" => Ok(Some(SpannedType {
                type_: Type::I8,
                start,
                end,
            })),
            "i16" => Ok(Some(SpannedType {
                type_: Type::I16,
                start,
                end,
            })),
            "i32" => Ok(Some(SpannedType {
                type_: Type::I32,
                start,
                end,
            })),
            "i64" => Ok(Some(SpannedType {
                type_: Type::I64,
                start,
                end,
            })),
            "f32" => Ok(Some(SpannedType {
                type_: Type::F32,
                start,
                end,
            })),
            "f64" => Ok(Some(SpannedType {
                type_: Type::F64,
                start,
                end,
            })),
            "bool" => Ok(Some(SpannedType {
                type_: Type::Bool,
                start,
                end,
            })),
            "char" => Ok(Some(SpannedType {
                type_: Type::Char,
                start,
                end,
            })),
            _ => todo!("Implement remaining types"),
        }

    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_int_literal() {
        let input = "42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::Literal(Literal::Int(42)),
            start: 0,
            end: 1,
        }));
    }

    #[test]
    fn test_parse_float_literal() {
        let input = "42.0;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::Literal(Literal::F64(42.0)),
            start: 0,
            end: 3,
        }));
    }

    #[test]
    fn test_parse_char_literal() {
        let input = "'a';";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::Literal(Literal::Char('a')),
            start: 0,
            end: 2,
        }));
    }

    #[test]
    fn test_parse_string_literal() {
        let input = "\"hello\";";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::Literal(Literal::String("hello".to_string())),
            start: 0,
            end: 6,
        }));
    }

    #[test]
    fn test_parse_bool_literal() {
        let input = "true;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::Literal(Literal::Bool(true)),
            start: 0,
            end: 3,
        }));
    }

    #[test]
    fn test_parse_addition_expression() {
        let input = "42 + 42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::BinaryExpression {
                left: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 0,
                    end: 1,
                }),
                operator: BinaryOperator::Add,
                right: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 5,
                    end: 6,
                }),
            },
            start: 0,
            end: 6,
        }));
    }

    #[test]
    fn test_parse_subtraction_expression() {
        let input = "42 - 42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::BinaryExpression {
                left: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 0,
                    end: 1,
                }),
                operator: BinaryOperator::Subtract,
                right: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 5,
                    end: 6,
                }),
            },
            start: 0,
            end: 6,
        }));
    }

    #[test]
    fn test_parse_multiplication_expression() {
        let input = "42 * 42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::BinaryExpression {
                left: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 0,
                    end: 1,
                }),
                operator: BinaryOperator::Multiply,
                right: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 5,
                    end: 6,
                }),
            },
            start: 0,
            end: 6,
        }));
    }

    #[test]
    fn test_parse_division_expression() {
        let input = "42 / 42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::BinaryExpression {
                left: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 0,
                    end: 1,
                }),
                operator: BinaryOperator::Divide,
                right: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 5,
                    end: 6,
                }),
            },
            start: 0,
            end: 6,
        }));
    }

    #[test]
    fn test_parse_modulo_expression() {
        let input = "42 % 42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::BinaryExpression {
                left: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 0,
                    end: 1,
                }),
                operator: BinaryOperator::Modulo,
                right: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 5,
                    end: 6,
                }),
            },
            start: 0,
            end: 6,
        }));
    }

    #[test]
    fn test_parse_negation_expression() {
        let input = "-42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::PrefixExpression {
                operator: PrefixOperator::Negate,
                right: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 1,
                    end: 2,
                }),
            },
            start: 0,
            end: 2,
        }));
    }

    #[test]
    fn test_parse_not_expression() {
        let input = "!true;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::PrefixExpression {
                operator: PrefixOperator::Not,
                right: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Bool(true)),
                    start: 1,
                    end: 4,
                }),
            },
            start: 0,
            end: 4,
        }));
    }

    #[test]
    fn test_parse_add_mult_expression() {
        let input = "42 + 42 * 42;";
        let mut parser = Parser::new(input);
        let result = parser.parse_statement();
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.statement, Statement::Expression(SpannedExpression {
            expression: Expression::BinaryExpression {
                left: Box::new(SpannedExpression {
                    expression: Expression::Literal(Literal::Int(42)),
                    start: 0,
                    end: 1,
                }),
                operator: BinaryOperator::Add,
                right: Box::new(SpannedExpression {
                    expression: Expression::BinaryExpression {
                        left: Box::new(SpannedExpression {
                            expression: Expression::Literal(Literal::Int(42)),
                            start: 5,
                            end: 6,
                        }),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(SpannedExpression {
                            expression: Expression::Literal(Literal::Int(42)),
                            start: 10,
                            end: 11,
                        }),
                    },
                    start: 5,
                    end: 11,
                }),
            },
            start: 0,
            end: 11,
        }));
    }
}
