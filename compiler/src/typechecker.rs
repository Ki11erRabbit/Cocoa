use std::collections::HashMap;

use either::Either;


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
    current_label: Option<String>,
    label_types: HashMap<String, ast::Type>,
}


impl TypeChecker {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            local_scope_stack: Vec::new(),
            current_label: None,
            label_types: HashMap::new(),
        }
    }

    fn bind_label(&mut self, name: &str, ty: ast::Type) {
        let name = name.to_string();
        self.label_types.insert(name, ty);
    }
    fn lookup_label(&self, name: &str) -> Option<ast::Type> {
        self.label_types.get(name).cloned()
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
                return Some(ty.clone());
            }
        }
        None
    }

    pub fn check_statements(&mut self, statements: Vec<crate::ast::SpannedStatement>, coerce_to: Option<ast::Type>) -> Result<Vec<ast::SpannedStatement>, ()> {
        self.push_scope();
        let mut checked_statements = Vec::new();
        for statement in statements {
            match self.check_statement(statement, coerce_to.clone()) {
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

    fn check_statement(&mut self, statement: crate::ast::SpannedStatement, coerce_to: Option<ast::Type>) -> Result<ast::SpannedStatement, TypeError> {
        let crate::ast::SpannedStatement { statement, start: stmt_start, end: stmt_end } = statement;
        match statement {
            crate::ast::Statement::Expression(expression) => {
                let expression_start = expression.start;
                let expression_end = expression.end;
                let (_, expr) = self.check_expression(expression, coerce_to)?;
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
                let (_, expr) = self.check_expression(expression, coerce_to)?;
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
                        let annotation: ast::Type = type_annotation.type_.into();
                        let expression_start = expression.start;
                        let expression_end = expression.end;
                        let (ty, expr) = self.check_expression(expression, Some(annotation.clone()))?;
                        self.bind_variable(name.clone(), ty.clone());
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
                let checked_body = match self.check_statements(body, None) {
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
            crate::ast::Statement::ForStatement { .. } => {
                todo!("Implement type checking for for loops")
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
                
                let (left_ty, left_expr) = self.check_expression(*left, coerce_to)?;
                let (right_ty, right_expr) = self.check_expression(*right, Some(left_ty.clone()))?;

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
                    crate::ast::BinaryOperator::ExclusiveRange |
                    crate::ast::BinaryOperator::InclusiveRange => {
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
                            Ok((ast::Type::Range(Box::new(left_ty)), ast::Expression::BinaryExpression {
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
            crate::ast::Expression::Label { name, body } => {
                self.bind_label(&name, coerce_to.clone().unwrap_or(ast::Type::Unit));
                let body = match body {
                    Either::Right(expr) => {
                        self.current_label = Some(name.clone());
                        let (ty, expr) = self.check_expression(*expr, coerce_to.clone())?;
                        self.current_label = None;
                        if let Some(cty) = coerce_to.clone() {
                            if ty != cty {
                                let error = TypeError::new(
                                    format!("Type mismatch: expected {}, found {}", cty, ty),
                                    expr_start,
                                    expr_end,
                                ).with_tip("Label body must have the same type as the label".to_string());
                                return Err(error);
                            }
                        }
                        ast::Expression::Label {
                            name,
                            body: Either::Right(Box::new(ast::SpannedExpression {
                                expression: expr,
                                start: expr_start,
                                end: expr_end,
                            })),
                        }
                    }
                    Either::Left(stmt) => {
                        let checked_stmt = match self.check_statement(*stmt, None) {
                            Ok(stmt) => stmt,
                            Err(error) => return Err(error),
                        };
                        ast::Expression::Label {
                            name,
                            body: Either::Left(Box::new(checked_stmt)),
                        }
                    }
                };
                match coerce_to {
                    Some(ty) => {
                        Ok((ty, body))
                    }
                    None => Ok((ast::Type::Unit, body)),
                }
            }
            crate::ast::Expression::LoopExpression { body } => {
                let ty = coerce_to.clone().unwrap_or(ast::Type::Unit);
                let checked_body = match self.check_statements(body, Some(ty.clone())) {
                    Ok(statements) => statements,
                    Err(()) => return Err(TypeError::new("Error in loop body".to_string(), expr_start, expr_end)),
                };

                let breaks = Self::find_breaks(&checked_body, &self.current_label);

                for break_ in breaks {
                    let ast::SpannedExpression { expression, start, end } = break_;
                    let ast::Expression::BreakExpression { type_, .. } = expression else {
                        panic!("Non-breaks found when only breaks should have been added");
                    };

                    let break_type = type_.clone().unwrap_or(ast::Type::Unit);
                    
                    if break_type != ty {
                        return Err(TypeError::new("Break statement type mismatch".to_string(), *start, *end));
                    }
                    
                }
                Ok((ty.clone(), ast::Expression::LoopExpression { type_: ty, body: checked_body}))
            }
            crate::ast::Expression::BreakExpression { label, expression } => {
                let label_ty = if let Some(label) = &label {
                    self.lookup_label(label).ok_or(TypeError::new("Label used but not declared.".to_string(), expr_start, expr_end))?.clone()
                } else {
                    coerce_to.clone().unwrap_or(ast::Type::Unit)
                };
                if let Some(expression) = expression {
                    let expression_start = expression.start;
                    let expression_end = expression.end;
                    let (ty, expr) = self.check_expression(*expression, Some(label_ty.clone()))?;
                    if label_ty != ty {
                        return Err(TypeError::new("Break type does not match loop type".to_string(), expr_start, expr_end))
                    }
                    Ok((ty.clone(), ast::Expression::BreakExpression {
                        type_: Some(ty),
                        label,
                        expression: Some(Box::new(ast::SpannedExpression {
                            expression: expr,
                            start: expression_start,
                            end: expression_end,
                        })),
                    }))
                } else {
                    Ok((ast::Type::Unit, ast::Expression::BreakExpression { type_: Some(label_ty), label, expression: None }))
                }
            }
            crate::ast::Expression::ReturnExpression(expr) => {
                // TODO: Implement checking of return type
                if let Some(expr) = expr {
                    let expression_start = expr.start;
                    let expression_end = expr.end;
                    let (ty, expr) = self.check_expression(*expr, None)?;
                    Ok((ty.clone(), ast::Expression::ReturnExpression(Some(Box::new(ast::SpannedExpression {
                        expression: expr,
                        start: expression_start,
                        end: expression_end,
                    }))))
                    )
                } else {
                    Ok((ast::Type::Unit, ast::Expression::ReturnExpression(None)))
                }
            }
            crate::ast::Expression::ContinueExpression(label) => {
                Ok((ast::Type::Unit, ast::Expression::ContinueExpression(label)))
            }
            _ => todo!("Implement type checking for other expressions"),
        }
    }


    fn find_breaks<'a>(body: &'a Vec<ast::SpannedStatement>, current_label: &Option<String>) -> Vec<&'a ast::SpannedExpression> {
        let mut loop_stack = Vec::new();
        let mut breaks = Vec::new();
        Self::explore_statements(body, current_label, &mut loop_stack, &mut breaks);
        breaks
    }

    fn explore_statements<'a>(body: &'a Vec<ast::SpannedStatement>, current_label: &Option<String>, loop_stack: &mut Vec<()>, breaks: &mut Vec<&'a ast::SpannedExpression>) {
        for statement in body {
            Self::explore_statement(statement, current_label, loop_stack, breaks);
        }
    }

    fn explore_statement<'a>(statement: &'a ast::SpannedStatement, current_label: &Option<String>, loop_stack: &mut Vec<()>, breaks: &mut Vec<&'a ast::SpannedExpression>) {
        let ast::SpannedStatement { statement, .. } = statement;
        match statement {
            ast::Statement::WhileStatement { body, .. } => {
                loop_stack.push(());
                Self::explore_statements(body, current_label, loop_stack, breaks);
                loop_stack.pop();
            }
            ast::Statement::Expression(expr) | ast::Statement::HangingExpression(expr) => {
                Self::explore_expression(expr, current_label, loop_stack, breaks);
            }
            _ => {}
        }
    }

    fn explore_expression<'a>(expr: &'a ast::SpannedExpression, current_label: &Option<String>, loop_stack: &mut Vec<()>, breaks: &mut Vec<&'a ast::SpannedExpression>) {
        let ast::SpannedExpression { expression, .. } = expr;
        match expression {
            ast::Expression::BreakExpression { label, .. } => {
                if let Some(label) = label {
                    if let Some(current_label) = current_label {
                        if label == current_label {
                            breaks.push(expr);
                        }
                    }
                } else {
                    if loop_stack.is_empty() {
                        breaks.push(expr);
                    }
                }
            }
            ast::Expression::LoopExpression { body, ..  } => {
                loop_stack.push(());
                Self::explore_statements(body, current_label, loop_stack, breaks);
                loop_stack.pop();
            }
            ast::Expression::Label { body, .. } => {
                match body {
                    Either::Left(stmt) => {
                        Self::explore_statement(stmt, current_label, loop_stack, breaks);
                    }
                    Either::Right(expr) => {
                        Self::explore_expression(expr.as_ref(), current_label, loop_stack, breaks);
                    }
                }
            }
            _ => {}
        }
    }


}
