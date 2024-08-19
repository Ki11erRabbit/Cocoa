mod parse_table;

use either::Either;

use crate::{ast::{BinaryOperator, Expression, Literal, Pattern, PrefixOperator, SpannedExpression, SpannedPattern, SpannedStatement, SpannedType, Statement, Type}, lexer::{SpannedToken, Token}};

use self::parse_table::ParseTable;

pub type ParseResult<'a, T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    Error {
        message: String,
        start: usize,
        end: usize,
    },
    EOF,
}

impl ParserError {
    pub fn new(message: &str, start: usize, end: usize) -> Self {
        ParserError::Error {
            message: message.to_string(),
            start,
            end,
        }
    }
}


pub struct Parser<'a> {
    parse_table: ParseTable<'a>,
}


impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            parse_table: ParseTable::new(input),
        }
    }

    pub fn next(&mut self) -> ParseResult<SpannedToken> {
        let next = self.parse_table.next();
        next
    }

    pub fn peek(&mut self) -> ParseResult<&SpannedToken> {
        self.parse_table.peek()
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

    pub fn parse_block(&mut self) -> ParseResult<Vec<SpannedStatement>> {
        let Ok(SpannedToken { start, end, .. }) = self.peek() else {
            panic!("Unexpected EOF");
        };
        let start = *start;
        let end = *end;
        let Ok(SpannedToken { token: Token::BraceOpen, .. }) = self.next() else {
            return Err(ParserError::new("Expected opening brace", start, end));
        };
        let mut body = Vec::new();
        loop {
            match self.peek() {
                Ok(SpannedToken { token: Token::BraceClose, .. }) => {
                    self.next()?;
                    break;
                }
                Ok(_) => {
                    match self.parse_statement() {
                        Ok(statement) => {
                            body.push(statement);
                        }
                        Err(ParserError::EOF) => {
                            return Err(ParserError::new("Unexpected EOF", body.last().unwrap().end, body.last().unwrap().end));
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                Err(ParserError::EOF) => {
                    return Err(ParserError::new("Unexpected EOF", body.last().unwrap().end, body.last().unwrap().end));
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(body)
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
                    Ok(SpannedToken { token: Token::Assign, .. }) => {
                        let SpannedToken { .. } = self.next()?;
                        let assign = expr?;
                        let start = assign.start;
                        let expr = self.parse_expression()?;
                        let end = expr.end;

                        let Ok(SpannedToken { token: Token::Semicolon, .. }) = self.next() else {
                            return Err(ParserError::new("Expected semicolon", end, end));
                        };
                        
                        return Ok(SpannedStatement {
                            statement: Statement::Assignment {
                                binding: assign,
                                expression: expr,
                            },
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
        if let Ok(SpannedToken { token: Token::Let, start, end }) = self.peek() {
            let start = *start;
            let end = *end;
            return self.parse_let_statement(start, end);
        }
        if let Ok(SpannedToken { token: Token::For, start, end }) = self.peek() {
            let start = *start;
            let end = *end;
            return self.parse_for_statement(start, end);
        }
        if let Ok(SpannedToken { token: Token::While, .. }) = self.peek() {
            return self.parse_while_statement();
        }

        todo!("Implement remaining statements")
    }

    fn parse_let_statement(&mut self, start: usize, end: usize) -> ParseResult<SpannedStatement> {
        let Ok(SpannedToken { token: Token::Let, .. }) = self.next() else {
            return Err(ParserError::new("Expected let keyword", start, end));
        };
        let pattern = self.parse_pattern()?;
        let ty = self.parse_annotated_type(false)?;
        let Ok(SpannedToken { start, end, .. }) = self.peek() else {
            panic!("Unexpected EOF");
        };
        let start = *start;
        let end = *end;
        let Ok(SpannedToken { token: Token::Assign, .. }) = self.next() else {
            return Err(ParserError::new("Expected assignment operator", start, end));
        };
        let expression = self.parse_expression()?;
        let start = expression.start;
        let end = expression.end;
        let Ok(SpannedToken { token: Token::Semicolon, end, start }) = self.next() else {
            return Err(ParserError::new("Expected semicolon", start, end));
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

}

// Expression Parsing
impl<'a> Parser<'a> {
    
    fn parse_expression_for_statement(&mut self) -> Option<ParseResult<SpannedExpression>> {
        match self.peek() {
            Ok(SpannedToken { token: Token::Let, .. }) |
            Ok(SpannedToken { token: Token::While, .. }) |
            Ok(SpannedToken { token: Token::For, .. }) |
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

        if let Ok(SpannedToken { token: Token::Label(_) , start, end }) = self.peek() {
            let start = *start;
            let end = *end;
            return self.parse_label_expression(start, end);
        }
        if let Ok(SpannedToken { token: Token::Loop, start, end }) = self.peek() {
            let start = *start;
            let end = *end;
            return self.parse_loop_expression(start, end);
        }
        if let Ok(SpannedToken { token: Token::Break, start, end }) = self.peek() {
            let start = *start;
            let end = *end;
            return self.parse_break_expression(start, end);
        }
        if let Ok(SpannedToken { token: Token::Return, start, end }) = self.peek() {
            let start = *start;
            let end = *end;
            let SpannedToken { token: Token::Return, .. } = self.next()? else {
                panic!("Expected return keyword after checking that it is a return keyword");
            };
            match self.parse_expression_for_statement() {
                None => {
                    return Ok(SpannedExpression {
                        expression: Expression::ReturnExpression(None),
                        start,
                        end,
                    });
                }
                Some(expr) => {
                    let expr = expr?;
                    let end = expr.end;
                    return Ok(SpannedExpression {
                        expression: Expression::ReturnExpression(Some(Box::new(expr))),
                        start,
                        end,
                    });
                }
            }
        }
        if let Ok(SpannedToken { token: Token::Continue, start, end }) = self.peek() {
            let start = *start;
            let end = *end;
            return self.parse_continue_expression(start, end);
        }
        // TODO parse if expression
        // TODO parse closure expression
        self.parse_range_expression()
    }

    fn parse_range_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse range expression
        let expr1 = self.parse_disjunction_expression()?;
        let op = match self.peek() {
            Ok(SpannedToken { token: Token::InclusiveRange, .. }) => {
                self.next()?;
                BinaryOperator::InclusiveRange
            }
            Ok(SpannedToken { token: Token::ExclusiveRange, .. }) => {
                self.next()?;
                BinaryOperator::ExclusiveRange
            }
            _ => return Ok(expr1),
        };
        let expr2 = self.parse_range_expression()?;
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

    fn parse_disjunction_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse disjunction expression

        self.parse_conjunction_expression()
    }

    fn parse_conjunction_expression(&mut self) -> ParseResult<SpannedExpression> {
        //TODO parse conjunction expression

        self.parse_equality_and_order_expression()
    }

    fn parse_equality_and_order_expression(&mut self) -> ParseResult<SpannedExpression> {

        let expr1 = self.parse_bitwise_or()?;
        let op = self.peek();
        let op = match op {
            Ok(SpannedToken { token: Token::Eq, .. }) => {
                self.next()?;
                BinaryOperator::Equal
            }
            Ok(SpannedToken { token: Token::Neq, .. }) => {
                self.next()?;
                BinaryOperator::NotEqual
            }
            Ok(SpannedToken { token: Token::Lt, .. }) => {
                self.next()?;
                BinaryOperator::LessThan
            }
            Ok(SpannedToken { token: Token::Gt, .. }) => {
                self.next()?;
                BinaryOperator::GreaterThan
            }
            Ok(SpannedToken { token: Token::Le, .. }) => {
                self.next()?;
                BinaryOperator::LessThanOrEqual
            }
            Ok(SpannedToken { token: Token::Ge, .. }) => {
                self.next()?;
                BinaryOperator::GreaterThanOrEqual
            }
            _ => return Ok(expr1),
        };
        let expr2 = self.parse_equality_and_order_expression()?;
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
            SpannedToken { token, start, end } => {
                println!("{:?}", token);
                let start = *start;
                let end = *end;
                Err(ParserError::new("Expected primary expression", start, end))
            }
        }
    }
}

// Loop Parsing
impl<'a> Parser<'a> {
    fn parse_while_statement(&mut self) -> ParseResult<SpannedStatement> {
        let Ok(SpannedToken { token: Token::While, start, end }) = self.next() else {
            panic!("Expected while keyword after checking that it is a while keyword");
        };
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        return Ok(SpannedStatement {
            statement: Statement::WhileStatement {
                condition,
                body,
            },
            start,
            end,
        });
    }

    fn parse_for_statement(&mut self, start: usize, end: usize) -> ParseResult<SpannedStatement> {
        let Ok(SpannedToken { token: Token::For, .. }) = self.next() else {
            return Err(ParserError::new("Expected for keyword", start, end));
        };
        let pattern = self.parse_pattern()?;
        let ty = self.parse_annotated_type(false)?;
        let Ok(SpannedToken { token: Token::In, .. }) = self.next() else {
            return Err(ParserError::new("Expected in keyword", pattern.end, pattern.end));
        };
        let expression = self.parse_expression()?;
        let body = self.parse_block()?;
        let body_end = body.last().unwrap().end;
        return Ok(SpannedStatement {
            statement: Statement::ForStatement {
                binding: pattern,
                typing: ty,
                expression,
                body,
            },
            start,
            end: body_end,
        });
    }

    fn parse_label_expression(&mut self, start: usize, end: usize) -> ParseResult<SpannedExpression> {
        let Ok(SpannedToken { token: Token::Label(label), .. }) = self.next() else {
            return Err(ParserError::new("Expected label", start, end));
        };
        let label = label.to_string();

        let Ok(SpannedToken { token: Token::Colon, .. }) = self.next() else {
            return Err(ParserError::new("Expected colon after label", start, end));
        };
        
        match self.parse_expression_for_statement() {
            None => {
                let statement = self.parse_statement()?;
                let end = statement.end;
                return Ok(SpannedExpression {
                    expression: Expression::Label {
                        name: label,
                        body: Either::Left(Box::new(statement)),
                    },
                    start,
                    end,
                });
            }
            Some(expr) => {
                let expr = expr?;
                let end = expr.end;
                return Ok(SpannedExpression {
                    expression: Expression::Label {
                        name: label,
                        body: Either::Right(Box::new(expr)),
                    },
                    start,
                    end,
                });
            }
        }
    }

    fn parse_break_expression(&mut self, start: usize, end: usize) -> ParseResult<SpannedExpression> {
        let Ok(SpannedToken { token: Token::Break, .. }) = self.next() else {
            return Err(ParserError::new("Expected break keyword", start, end));
        };
        let (label, label_end) = if let Ok(SpannedToken { token: Token::Label(_), .. }) = self.peek() {
            let SpannedToken { token: Token::Label(label), end, .. } = self.next()? else {
                panic!("Expected label after checking that it is a label");
            };
            (Some(label.to_string()), end)
        } else {
            (None, end)
        };
        let (expression, expression_end) = if let Ok(SpannedToken { token: Token::Semicolon, .. }) = self.peek() {
            (None, label_end)
        } else {
            let expr = self.parse_expression()?;
            let end = expr.end;
            (Some(Box::new(expr)), end)
        };

        Ok(SpannedExpression {
            expression: Expression::BreakExpression {
                label,
                expression,
            },
            start,
            end: expression_end,
        })
    }

    fn parse_loop_expression(&mut self, start: usize, end: usize) -> ParseResult<SpannedExpression> {
        let Ok(SpannedToken { token: Token::Loop, .. }) = self.next() else {
            return Err(ParserError::new("Expected loop keyword", start, end));
        };
        let body = self.parse_block()?;
        let body_end = body.last().unwrap().end;
        Ok(SpannedExpression {
            expression: Expression::LoopExpression {
                body,
            },
            start,
            end: body_end,
        })
    }

    fn parse_continue_expression(&mut self, start: usize, end: usize) -> ParseResult<SpannedExpression> {
        let Ok(SpannedToken { token: Token::Continue, .. }) = self.next() else {
            return Err(ParserError::new("Expected continue keyword", start, end));
        };
        let (label, label_end) = if let Ok(SpannedToken { token: Token::Label(_), .. }) = self.peek() {
            let SpannedToken { token: Token::Label(label), end, .. } = self.next()? else {
                panic!("Expected label after checking that it is a label");
            };
            (Some(label.to_string()), end)
        } else {
            (None, end)
        };
        Ok(SpannedExpression {
            expression: Expression::ContinueExpression(label),
            start,
            end: label_end,
        })
    }
}


// Type Parsing
impl<'a> Parser<'a> {
    fn parse_annotated_type(&mut self, fail_on_missing: bool) -> ParseResult<Option<SpannedType>> {
        let Ok(SpannedToken { start, end, .. }) = self.peek() else {
            panic!("Unexpected EOF");
        };
        let start = *start;
        let end = *end;

        let Ok(SpannedToken { token: Token::Colon, .. }) = self.peek() else {
            if fail_on_missing {
                return Err(ParserError::new("Expected type annotation", start, end));
            }
            return Ok(None);
        };
        Ok(Some(self.parse_type()?))
    }
    
    fn parse_type(&mut self) -> ParseResult<SpannedType> {
        let SpannedToken { end, start, .. } = self.next()?;
        let Ok(SpannedToken { token: _, start, end }) = self.peek() else {
            return Err(ParserError::new("Expected type identifier", start, end));
        };
        let start = *start;
        let end = *end;
        let Ok(SpannedToken { token: Token::TypeIdentifier(id), start, end }) = self.next() else {
            return Err(ParserError::new("Expected type identifier", start, end));
        };
        match id {
            "u8" => Ok(SpannedType {
                type_: Type::U8,
                start,
                end,
            }),
            "u16" => Ok(SpannedType {
                type_: Type::U16,
                start,
                end,
            }),
            "u32" => Ok(SpannedType {
                type_: Type::U32,
                start,
                end,
            }),
            "u64" => Ok(SpannedType {
                type_: Type::U64,
                start,
                end,
            }),
            "i8" => Ok(SpannedType {
                type_: Type::I8,
                start,
                end,
            }),
            "i16" => Ok(SpannedType {
                type_: Type::I16,
                start,
                end,
            }),
            "i32" => Ok(SpannedType {
                type_: Type::I32,
                start,
                end,
            }),
            "i64" => Ok(SpannedType {
                type_: Type::I64,
                start,
                end,
            }),
            "f32" => Ok(SpannedType {
                type_: Type::F32,
                start,
                end,
            }),
            "f64" => Ok(SpannedType {
                type_: Type::F64,
                start,
                end,
            }),
            "bool" => Ok(SpannedType {
                type_: Type::Bool,
                start,
                end,
            }),
            "char" => Ok(SpannedType {
                type_: Type::Char,
                start,
                end,
            }),
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
