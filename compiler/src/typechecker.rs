use std::collections::HashMap;

pub mod ast;



#[derive(Debug)]
pub struct TypeError {
    pub message: String,
    pub tip: Option<String>,
    pub start: usize,
    pub end: usize,
    pub additional: Vec<(usize, usize, String)>,
}

impl TypeError {
    pub fn new(message: String, start: usize, end: usize) -> Self {
        Self {
            message,
            tip: None,
            start,
            end,
            additional: Vec::new(),
        }
    }
    pub fn with_tip(mut self, tip: String) -> Self {
        self.tip = Some(tip);
        self
    }

    pub fn with_additional(mut self, start: usize, end: usize, message: String) -> Self {
        self.additional.push((start, end, message));
        self
    }
}

pub struct TypeChecker {
    pub errors: Vec<TypeError>,
    locals: HashMap<String, ast::Type>,
}


impl TypeChecker {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            locals: HashMap::new(),
        }
    }

    pub fn check_statements(&mut self, statements: Vec<crate::ast::SpannedStatement>) -> Result<Vec<ast::SpannedStatement>, ()> {
        let mut checked_statements = Vec::new();
        for statement in statements {
            match self.check_statement(statement) {
                Ok(statement) => checked_statements.push(statement),
                Err(error) => self.errors.push(error),
            }
        }
        if self.errors.is_empty() {
            Ok(checked_statements)
        } else {
            Err(())
        }
    }

    fn check_statement(&mut self, statement: crate::ast::SpannedStatement) -> Result<ast::SpannedStatement, TypeError> {
        let crate::ast::SpannedStatement { statement, start: stmt_start, end: stmt_end } = statement;
        match statement {
            crate::ast::Statement::Expression(expression) => {
                let expression_start = expression.start;
                let expression_end = expression.end;
                let (_, expr) = self.check_expression(expression)?;
                Ok(ast::SpannedStatement {
                    statement: ast::Statement::Expression(ast::SpannedExpression {
                        expression: expr,
                        start: expression_start,
                        end: expression_end,
                    }),
                    start: stmt_start,
                    end: stmt_end,
                })
            }
            crate::ast::Statement::HangingExpression(expression) => {
                let expression_start = expression.start;
                let expression_end = expression.end;
                let (_, expr) = self.check_expression(expression)?;
                Ok(ast::SpannedStatement {
                    statement: ast::Statement::HangingExpression(ast::SpannedExpression {
                        expression: expr,
                        start: expression_start,
                        end: expression_end,
                    }),
                    start: stmt_start,
                    end: stmt_end,
                })
            }
            crate::ast::Statement::LetStatement { binding, type_annotation, expression } => {
                let crate::ast::SpannedPattern { pattern, start: pattern_start, end: pattern_end } = binding;
                if let crate::ast::Pattern::Identifier(name) = pattern {
                    let expression_start = expression.start;
                    let expression_end = expression.end;
                    let (ty, expr) = self.check_expression(expression)?;

                    if let Some(type_annotation) = type_annotation {
                        let annotation = type_annotation.type_.into();
                        self.locals.insert(name.clone(), annotation);
                        if ty != annotation {
                            let type_error = TypeError::new(
                                format!("Type mismatch: expected {}, found {}", annotation, ty),
                                pattern_start,
                                pattern_end,
                            ).with_tip(format!("Change type annotation."))
                                .with_additional(type_annotation.start, type_annotation.end, "Type annotation here".to_string())
                                .with_additional(expression_start, expression_end, "Expression here".to_string());
                            return Err(type_error);
                        }
                        self.locals.insert(name.clone(), ty);

                        Ok(ast::SpannedStatement {
                            statement: ast::Statement::LetStatement {
                                binding: ast::SpannedPattern {
                                    pattern: ast::Pattern::Identifier(name),
                                    start: pattern_start,
                                    end: pattern_end,
                                },
                                type_annotation: ast::SpannedType {
                                    type_: annotation,
                                    start: type_annotation.start,
                                    end: type_annotation.end,
                                },
                                expression: ast::SpannedExpression {
                                    expression: expr,
                                    start: expression_start,
                                    end: expression_end,
                                },
                            },
                            start: stmt_start,
                            end: stmt_end,
                        })
                    } else {
                        todo!("Perform type inference here")
                    }
                } else {
                    todo!("Implement type checking for other patterns")
                }
            }
        }
    }

    fn check_expression(&mut self, expression: crate::ast::SpannedExpression) -> Result<(ast::Type, ast::Expression), TypeError> {
        let crate::ast::SpannedExpression { expression, start: expr_start, end: expr_end } = expression;
        match expression {
            crate::ast::Expression::Literal(lit) => {
                match lit {
                    crate::ast::Literal::U8(lit) => Ok((ast::Type::U8, ast::Expression::Literal(ast::Literal::U8(lit)))),
                    crate::ast::Literal::U16(lit) => Ok((ast::Type::U16, ast::Expression::Literal(ast::Literal::U16(lit)))),
                    crate::ast::Literal::U32(lit) => Ok((ast::Type::U32, ast::Expression::Literal(ast::Literal::U32(lit)))),
                    crate::ast::Literal::U64(lit) => Ok((ast::Type::U64, ast::Expression::Literal(ast::Literal::U64(lit)))),
                    crate::ast::Literal::I8(lit) => Ok((ast::Type::I8, ast::Expression::Literal(ast::Literal::I8(lit)))),
                    crate::ast::Literal::I16(lit) => Ok((ast::Type::I16, ast::Expression::Literal(ast::Literal::I16(lit)))),
                    crate::ast::Literal::I32(lit) => Ok((ast::Type::I32, ast::Expression::Literal(ast::Literal::I32(lit)))),
                    crate::ast::Literal::I64(lit) => Ok((ast::Type::I64, ast::Expression::Literal(ast::Literal::I64(lit)))),
                    crate::ast::Literal::F32(lit) => Ok((ast::Type::F32, ast::Expression::Literal(ast::Literal::F32(lit)))),
                    crate::ast::Literal::F64(lit) => Ok((ast::Type::F64, ast::Expression::Literal(ast::Literal::F64(lit)))),
                    crate::ast::Literal::Bool(lit) => Ok((ast::Type::Bool, ast::Expression::Literal(ast::Literal::Bool(lit)))),
                    crate::ast::Literal::Char(lit) => Ok((ast::Type::Char, ast::Expression::Literal(ast::Literal::Char(lit)))),
                    crate::ast::Literal::Int(lit) => todo!("Implement type checking for integer literals"),
                    _ => todo!("Implement type checking for other literals"),
                }
            }
            crate::ast::Expression::Variable(ident) => {
                if let Some(ty) = self.locals.get(&ident) {
                    Ok((*ty, ast::Expression::Variable(ident)))
                } else {
                    let error = TypeError::new(
                        format!("Variable {} not found in this scope", ident),
                        expr_start,
                        expr_end,
                    );
                    Err(error)
                }
            }
            crate::ast::Expression::BinaryExpression { left, operator, right } => {
                let left_start = left.start;
                let left_end = left.end;
                let right_start = right.start;
                let right_end = right.end;
                
                let (left_ty, left_expr) = self.check_expression(*left)?;
                let (right_ty, right_expr) = self.check_expression(*right)?;

                //TODO: add lookup for trait impls to allow for overloading
                match operator {
                    crate::ast::BinaryOperator::Add | 
                    crate::ast::BinaryOperator::Subtract |
                    crate::ast::BinaryOperator::Multiply |
                    crate::ast::BinaryOperator::Divide |
                    crate::ast::BinaryOperator::Modulo => {
                        if left_ty == right_ty {
                            let spanned_left = ast::SpannedExpression {
                                expression: left_expr,
                                start: left_start,
                                end: left_end,
                            };
                            let spanned_right = ast::SpannedExpression {
                                expression: right_expr,
                                start: right_start,
                                end: right_end,
                            };
                            let operator = operator.into();
                            Ok((left_ty, ast::Expression::BinaryExpression {
                                left: Box::new(spanned_left),
                                operator,
                                right: Box::new(spanned_right),
                            }))
                        } else {
                            let operator: ast::BinaryOperator = operator.into();
                            let error = TypeError::new(
                                format!("Type mismatch: expected {}, found {}", left_ty, right_ty),
                                expr_start,
                                expr_end,
                            ).with_tip(format!("Both operands must have the same type for {}", operator))
                                .with_additional(left_start, left_end, "Left operand here".to_string())
                                .with_additional(right_start, right_end, "Right operand here".to_string());
                            Err(error)
                        }
                    }
                    _ => todo!("Implement type checking for other binary operators"),
                }
            }
            _ => todo!("Implement type checking for other expressions"),
        }
    }
}
