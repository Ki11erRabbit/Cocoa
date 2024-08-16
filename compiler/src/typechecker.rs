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
    local_scope_stack: Vec<HashMap<String, ast::Type>>,
}


impl TypeChecker {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            local_scope_stack: Vec::new(),
        }
    }

    fn push_scope(&mut self) {
        self.local_scope_stack.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.local_scope_stack.pop();
    }

    fn bind_variable(&mut self, name: String, ty: ast::Type) {
        self.local_scope_stack.last_mut().unwrap().insert(name, ty);
    }

    fn lookup_variable(&self, name: &str) -> Option<ast::Type> {
        for scope in self.local_scope_stack.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(*ty);
            }
        }
        None
    }

    pub fn check_statements(&mut self, statements: Vec<crate::ast::SpannedStatement>) -> Result<Vec<ast::SpannedStatement>, ()> {
        self.push_scope();
        let mut checked_statements = Vec::new();
        for statement in statements {
            match self.check_statement(statement) {
                Ok(statement) => checked_statements.push(statement),
                Err(error) => self.errors.push(error),
            }
        }
        self.pop_scope();
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
                let (_, expr) = self.check_expression(expression, None)?;
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
                let (_, expr) = self.check_expression(expression, None)?;
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

                    if let Some(type_annotation) = type_annotation {
                        let annotation = type_annotation.type_.into();
                        let expression_start = expression.start;
                        let expression_end = expression.end;
                        let (ty, expr) = self.check_expression(expression, Some(annotation))?;
                        self.bind_variable(name.clone(), ty);
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
                        self.bind_variable(name.clone(), ty);

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
            crate::ast::Statement::Assignment { binding, expression } => {
                let binding_start = binding.start;
                let binding_end = binding.end;
                let expression_start = expression.start;
                let expression_end = expression.end;
                let (binding_ty, binding) = self.check_binding(binding)?;
                let (expr_ty, expr) = self.check_expression(expression, None)?;
                if binding_ty != expr_ty {
                    let error = TypeError::new(
                        format!("Type mismatch: expected {}, found {}", binding_ty, expr_ty),
                        binding_start,
                        binding_end,
                    ).with_tip("Both sides of the assignment must have the same type".to_string())
                    .with_additional(binding_start, binding_end, "Binding here".to_string())
                        .with_additional(expression_start, expression_end, "Expression here".to_string());
                    return Err(error);
                }

                Ok(ast::SpannedStatement {
                    statement: ast::Statement::Assignment {
                        binding: ast::SpannedLhs {
                            lhs: binding,
                            start: binding_start,
                            end: binding_end,
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
                
            }
            crate::ast::Statement::WhileStatement { condition, body } => {
                let condition_start = condition.start;
                let condition_end = condition.end;
                let (ty, cond) = self.check_expression(condition, None)?;
                if ty != ast::Type::Bool {
                    let error = TypeError::new(
                        format!("Type mismatch: expected bool, found {}", ty),
                        condition_start,
                        condition_end,
                    ).with_tip("While condition must be a boolean expression".to_string());
                    return Err(error);
                }
                let checked_body = match self.check_statements(body) {
                    Ok(statements) => statements,
                    Err(()) => return Err(TypeError::new("Error in while loop body".to_string(), stmt_start, stmt_end)),
                };
                Ok(ast::SpannedStatement {
                    statement: ast::Statement::WhileStatement {
                        condition: ast::SpannedExpression {
                            expression: cond,
                            start: condition_start,
                            end: condition_end,
                        },
                        body: checked_body,
                    },
                    start: stmt_start,
                    end: stmt_end,
                })
            }
        }
    }

    fn check_binding(&mut self, expression: crate::ast::SpannedExpression) -> Result<(ast::Type, ast::Lhs), TypeError> {
        let crate::ast::SpannedExpression { expression, start: expr_start, end: expr_end } = expression;
        match expression {
            crate::ast::Expression::Variable(ident) => {
                if let Some(ty) = self.lookup_variable(&ident) {
                    Ok((ty, ast::Lhs::Variable(ident)))
                } else {
                    let error = TypeError::new(
                        format!("Variable {} not found in this scope", ident),
                        expr_start,
                        expr_end,
                    );
                    Err(error)
                }
            }
            _ => todo!("Implement binding for other expressions"),
        }
    }

    fn check_expression(&mut self, expression: crate::ast::SpannedExpression, coerce_to: Option<ast::Type>) -> Result<(ast::Type, ast::Expression), TypeError> {
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
                    crate::ast::Literal::Int(lit) => match coerce_to {
                        Some(ast::Type::I8) => Ok((ast::Type::I8, ast::Expression::Literal(ast::Literal::I8(lit as i8)))),
                        Some(ast::Type::I16) => Ok((ast::Type::I16, ast::Expression::Literal(ast::Literal::I16(lit as i16)))),
                        Some(ast::Type::I32) => Ok((ast::Type::I32, ast::Expression::Literal(ast::Literal::I32(lit as i32)))),
                        Some(ast::Type::I64) => Ok((ast::Type::I64, ast::Expression::Literal(ast::Literal::I64(lit as i64)))),
                        Some(ast::Type::U8) => Ok((ast::Type::U8, ast::Expression::Literal(ast::Literal::U8(lit as u8)))),
                        Some(ast::Type::U16) => Ok((ast::Type::U16, ast::Expression::Literal(ast::Literal::U16(lit as u16)))),
                        Some(ast::Type::U32) => Ok((ast::Type::U32, ast::Expression::Literal(ast::Literal::U32(lit as u32)))),
                        Some(ast::Type::U64) => Ok((ast::Type::U64, ast::Expression::Literal(ast::Literal::U64(lit as u64)))),
                        Some(ast::Type::F32) => Ok((ast::Type::F32, ast::Expression::Literal(ast::Literal::F32(lit as f32)))),
                        Some(ast::Type::F64) => Ok((ast::Type::F64, ast::Expression::Literal(ast::Literal::F64(lit as f64)))),
                        None => Ok((ast::Type::I32, ast::Expression::Literal(ast::Literal::I32(lit as i32)))),
                        _ => todo!("Implement type coercion for other types"),
                    }
                    _ => todo!("Implement type checking for other literals"),
                }
            }
            crate::ast::Expression::Variable(ident) => {
                if let Some(ty) = self.lookup_variable(&ident) {
                    Ok((ty, ast::Expression::Variable(ident)))
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
                
                let (left_ty, left_expr) = self.check_expression(*left, None)?;
                let (right_ty, right_expr) = self.check_expression(*right, Some(left_ty))?;

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
                    crate::ast::BinaryOperator::Equal |
                    crate::ast::BinaryOperator::NotEqual => {
                        // TODO: Implement checking of trait impls for equality
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
                            Ok((ast::Type::Bool, ast::Expression::BinaryExpression {
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
                    crate::ast::BinaryOperator::LessThan |
                    crate::ast::BinaryOperator::LessThanOrEqual |
                    crate::ast::BinaryOperator::GreaterThan |
                    crate::ast::BinaryOperator::GreaterThanOrEqual => {
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
                            Ok((ast::Type::Bool, ast::Expression::BinaryExpression {
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
