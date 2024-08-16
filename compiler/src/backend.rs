
use std::collections::HashMap;

use bytecode::Bytecode;

use crate::typechecker::ast::{BinaryOperator, Expression, Lhs, Pattern, PrefixOperator, SpannedExpression, SpannedStatement, Statement};


pub trait IntoBinary {
    fn into_binary(&self) -> Vec<u8>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    Object,
    Str,
}


impl From<Value> for Type {
    fn from(value: Value) -> Self {
        match value {
            Value::U8(_) => Type::U8,
            Value::U16(_) => Type::U16,
            Value::U32(_) => Type::U32,
            Value::U64(_) => Type::U64,
            Value::I8(_) => Type::I8,
            Value::I16(_) => Type::I16,
            Value::I32(_) => Type::I32,
            Value::I64(_) => Type::I64,
            Value::F32(_) => Type::F32,
            Value::F64(_) => Type::F64,
            Value::Char(_) => Type::Char,
            Value::Object => Type::Object,
            Value::Str => Type::Str,
        }
    }
}

impl From<Type> for u8 {
    fn from(ty: Type) -> Self {
        match ty {
            Type::U8 => 0,
            Type::U16 => 1,
            Type::U32 => 2,
            Type::U64 => 3,
            Type::I8 => 4,
            Type::I16 => 5,
            Type::I32 => 6,
            Type::I64 => 7,
            Type::F32 => 8,
            Type::F64 => 9,
            Type::Char => 10,
            Type::Object => 11,
            Type::Str => 12,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    Object,
    Str,
}

impl From<Type> for Value {
    fn from(ty: Type) -> Self {
        match ty {
            Type::U8 => Value::U8(0),
            Type::U16 => Value::U16(0),
            Type::U32 => Value::U32(0),
            Type::U64 => Value::U64(0),
            Type::I8 => Value::I8(0),
            Type::I16 => Value::I16(0),
            Type::I32 => Value::I32(0),
            Type::I64 => Value::I64(0),
            Type::F32 => Value::F32(0.0),
            Type::F64 => Value::F64(0.0),
            Type::Char => Value::Char('\0'),
            Type::Object => Value::Object,
            Type::Str => Value::Str,
        }
    }
}

impl IntoBinary for Value {
    fn into_binary(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Value::U8(value) => {
                bytes.push(0);
                bytes.push(*value);
            }
            Value::U16(value) => {
                bytes.push(1);
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            Value::U32(value) => {
                bytes.push(2);
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            Value::U64(value) => {
                bytes.push(3);
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            Value::I8(value) => {
                bytes.push(4);
                bytes.push(*value as u8);
            }
            Value::I16(value) => {
                bytes.push(5);
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            Value::I32(value) => {
                bytes.push(6);
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            Value::I64(value) => {
                bytes.push(7);
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            Value::F32(value) => {
                bytes.push(8);
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            Value::F64(value) => {
                bytes.push(9);
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            Value::Char(value) => {
                bytes.push(10);
                let mut buffer = [0; 8];
                let encoded = value.encode_utf8(&mut buffer);
                let length = encoded.len() as u64;
                bytes.extend_from_slice(&length.to_le_bytes());
                bytes.extend_from_slice(&encoded.as_bytes());
            }
            _ => todo!(),
        }
        bytes
    }
}

pub struct ConstantPool {
    constants: Vec<Value>,
}

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool {
            constants: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> u64 {
        self.constants.push(value);
        (self.constants.len() - 1) as u64
    }

    pub fn get_constant(&self, index: u64) -> Value {
        self.constants[index as usize]
    }
}

impl IntoBinary for ConstantPool {
    fn into_binary(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.constants.len() as u64).to_le_bytes());
        for constant in &self.constants {
            bytes.extend_from_slice(&constant.into_binary());
        }
        bytes
    }
}

pub struct Frame {
    operands: Vec<Value>,
    locals: [Value; 256],
    next_local: u8,
    name_to_local: HashMap<(String, Type), u8>,
    name_to_position: HashMap<String, (String, Type)>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            operands: Vec::new(),
            locals: [Value::U8(0); 256],
            next_local: 0,
            name_to_local: HashMap::new(),
            name_to_position: HashMap::new(),
        }
    }

    pub fn push_value(&mut self, value: Value) {
        self.operands.push(value);
    }

    pub fn pop_value(&mut self) -> Value {
        self.operands.pop().unwrap()
    }

    pub fn store_local(&mut self, name: &str, value: Value) -> u8 {
        let local = self.next_local;
        self.next_local += 1;
        self.locals[local as usize] = value;
        self.name_to_position.insert(name.to_string(), (local.to_string(), value.into()));
        self.name_to_local.insert((name.to_string(), value.into()), local);
        local
    }

    pub fn load_local(&self, name: &str) -> (u8, Value) {
        let (local, ty) = self.name_to_position.get(name).unwrap();
        let local = self.name_to_local.get(&(local.to_string(), *ty)).unwrap();
        (*local, self.locals[*local as usize])
    }
}

pub struct Stack {
    frames: Vec<Frame>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            frames: vec![Frame::new()],
        }
    }

    pub fn push_frame(&mut self) {
        self.frames.push(Frame::new());
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn push_value(&mut self, value: Value) {
        self.frames.last_mut().unwrap().push_value(value);
    }

    pub fn pop_value(&mut self) -> Value {
        self.frames.last_mut().unwrap().pop_value()
    }
}


pub struct StatementsCompiler {
    stack: Stack,
    bytecode: Vec<Bytecode>,
    block_count: u64,
}


impl StatementsCompiler {
    pub fn new() -> Self {
        StatementsCompiler {
            stack: Stack::new(),
            bytecode: Vec::new(),
            block_count: 0,
        }
    }

    fn add_block(&mut self) -> u64 {
        self.block_count += 1;
        self.block_count
    }

    fn current_block(&self) -> u64 {
        self.block_count
    }

    fn bind_local(&mut self, name: &str, ty: Type) {
        let index = self.stack.frames.last_mut().unwrap().store_local(name, ty.into());
        self.bytecode.push(Bytecode::StoreLocal(index, ty.into()));
    }

    fn lookup_local(&mut self, name: &str) -> Value {
        let (index, value) = self.stack.frames.last().unwrap().load_local(name);
        self.bytecode.push(Bytecode::LoadLocal(index));
        value
    }

    pub fn compile_statements(&mut self, constant_pool: &mut ConstantPool, statements: &[SpannedStatement]) {
        self.bytecode.push(Bytecode::StartBlock(0));
        for statement in statements {
            println!("{:?}", statement);
            self.compile_statement(constant_pool, statement);
        }
    }

    pub fn compile_statement(&mut self, constant_pool: &mut ConstantPool, statement: &SpannedStatement) {
        match &statement.statement {
            Statement::Expression(expr) => {
                self.compile_expression(constant_pool, expr);
            }
            Statement::HangingExpression(expr) => {
                self.compile_expression(constant_pool, expr);
                //TODO: Check that it is at the end of a function
                self.bytecode.push(Bytecode::Return);
            }
            Statement::LetStatement { binding, expression, .. } => {
                if let Pattern::Identifier(name) = &binding.pattern {
                    let ty = self.compile_expression(constant_pool, expression);
                    self.bind_local(name, ty);
                }
            }
            Statement::Assignment { binding, expression } => {
                let ty = self.compile_expression(constant_pool, expression);
                match &binding.lhs {
                    Lhs::Variable(name) => {
                        self.bind_local(name, ty);
                    }
                }
            }
            Statement::WhileStatement { condition, body } => {
                self.compile_while_statement(constant_pool, condition, body);
            }
        }
    }

    fn compile_while_statement(&mut self, constant_pool: &mut ConstantPool, condition: &SpannedExpression, body: &[SpannedStatement]) {
        let condition_block = self.add_block();
        self.bytecode.push(Bytecode::Goto(condition_block));
        self.bytecode.push(Bytecode::StartBlock(condition_block));
        self.compile_while_conditional(constant_pool, condition);

        self.stack.push_frame();
        let body_block = self.add_block();
        self.bytecode.push(Bytecode::StartBlock(body_block));
        for statement in body {
            self.compile_statement(constant_pool, statement);
        }
        self.stack.pop_frame();

        self.bytecode.push(Bytecode::Goto(condition_block));
        let next_block = self.add_block();
        self.bytecode.push(Bytecode::StartBlock(next_block));
    }

    fn compile_while_conditional(&mut self, constant_pool: &mut ConstantPool, condition: &SpannedExpression) {
        self.compile_expression(constant_pool, condition);
        let if_instruction = Bytecode::If(self.current_block() + 1, self.current_block() + 2);
        self.bytecode.push(if_instruction);
    }

    fn compile_expression(&mut self, constant_pool: &mut ConstantPool, expr: &SpannedExpression) -> Type {
        match &expr.expression {
            Expression::BinaryExpression { left, operator, right } => {
                let ty1 = self.compile_expression(constant_pool, left);
                let _ = self.compile_expression(constant_pool, right);
                // TODO: Check that the types are compatible
                match operator {
                    BinaryOperator::Add => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Addu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Addu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Addu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Addu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Addi8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Addi16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Addi32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Addi64);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Addf32);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Addf64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::Subtract => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Subu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Subu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Subu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Subu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Subi8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Subi16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Subi32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Subi64);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Subf32);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Subf64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::Multiply => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Mulu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Mulu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Mulu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Mulu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Muli8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Muli16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Muli32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Muli64);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Mulf32);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Mulf64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::Divide => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Divu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Divu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Divu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Divu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Divi8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Divi16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Divi32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Divi64);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Divf32);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Divf64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::Modulo => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Modu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Modu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Modu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Modu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Modi8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Modi16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Modi32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Modi64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::Equal => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Equalu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Equalu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Equalu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Equalu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Equali8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Equali16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Equali32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Equali64);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Equalf32);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Equalf64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::Or => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Oru16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Oru32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Oru64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Ori8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Ori16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Ori32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Ori64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::And => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Andu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Andu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Andu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Andu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Andi8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Andi16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Andi32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Andi64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::LessThan => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Lessu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Lessu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Lessu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Lessu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Lessi8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Lessi16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Lessi32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Lessi64);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Lessf32);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Lessf64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::LessThanOrEqual => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessu8);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalu8);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessu16);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalu16);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessu32);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalu32);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessu64);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalu64);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessi8);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equali8);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessi16);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equali16);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessi32);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equali32);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessi64);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equali64);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessf32);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalf32);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Lessf64);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalf64);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::GreaterThan => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Greateru8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Greateru16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Greateru32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Greateru64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Greateri8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Greateri16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Greateri32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Greateri64);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Greaterf32);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Greaterf64);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::GreaterThanOrEqual => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greateru8);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalu8);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greateru16);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalu16);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greateru32);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalu32);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greateru64);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalu64);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greateri8);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equali8);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greateri16);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equali16);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greateri32);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equali32);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greateri64);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equali64);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greaterf32);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalf32);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Dup);
                                self.bytecode.push(Bytecode::Greaterf64);
                                self.bytecode.push(Bytecode::Swap);
                                self.bytecode.push(Bytecode::Equalf64);
                                self.bytecode.push(Bytecode::Oru8);
                            }
                            _ => {}
                        }
                    }
                    BinaryOperator::NotEqual => {
                        match ty1 {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Equalu8);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Equalu16);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Equalu32);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Equalu64);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Equali8);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Equali16);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Equali32);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Equali64);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Equalf32);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Equalf64);
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            _ => {}
                        }
                    }
                    _ => todo!("binary operator {:?}", operator),
                }
                ty1
            }
            Expression::PrefixExpression { operator, right } => {
                let ty = self.compile_expression(constant_pool, right);
                match operator {
                    PrefixOperator::Negate => {
                        match ty {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Negu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Negu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Negu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Negu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Negi8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Negi16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Negi32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Negi64);
                            }
                            Type::F32 => {
                                self.bytecode.push(Bytecode::Negf32);
                            }
                            Type::F64 => {
                                self.bytecode.push(Bytecode::Negf64);
                            }
                            _ => {}
                        }
                    }
                    PrefixOperator::Not => {
                        match ty {
                            Type::U8 => {
                                self.bytecode.push(Bytecode::Notu8);
                            }
                            Type::U16 => {
                                self.bytecode.push(Bytecode::Notu16);
                            }
                            Type::U32 => {
                                self.bytecode.push(Bytecode::Notu32);
                            }
                            Type::U64 => {
                                self.bytecode.push(Bytecode::Notu64);
                            }
                            Type::I8 => {
                                self.bytecode.push(Bytecode::Noti8);
                            }
                            Type::I16 => {
                                self.bytecode.push(Bytecode::Noti16);
                            }
                            Type::I32 => {
                                self.bytecode.push(Bytecode::Noti32);
                            }
                            Type::I64 => {
                                self.bytecode.push(Bytecode::Noti64);
                            }
                            _ => {}
                        }
                    }
                }
                ty
            }
            Expression::PostfixExpression { left, operator } => {
                let _ty = self.compile_expression(constant_pool, left);
                match operator {
                    _ => todo!("postfix operator {:?}", operator),
                }
            }
            Expression::Literal(literal) => {
                match literal {
                    crate::typechecker::ast::Literal::U8(value) => {
                        let index = constant_pool.add_constant(Value::U8(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U8
                    }
                    crate::typechecker::ast::Literal::U16(value) => {
                        let index = constant_pool.add_constant(Value::U16(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U16
                    }
                    crate::typechecker::ast::Literal::U32(value) => {
                        let index = constant_pool.add_constant(Value::U32(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U32
                    }
                    crate::typechecker::ast::Literal::U64(value) => {
                        let index = constant_pool.add_constant(Value::U64(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U64
                    }
                    crate::typechecker::ast::Literal::I8(value) => {
                        let index = constant_pool.add_constant(Value::I8(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::I8
                    }
                    crate::typechecker::ast::Literal::I16(value) => {
                        let index = constant_pool.add_constant(Value::I16(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::I16
                    }
                    crate::typechecker::ast::Literal::I32(value) => {
                        let index = constant_pool.add_constant(Value::I32(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::I32
                    }
                    crate::typechecker::ast::Literal::I64(value) => {
                        let index = constant_pool.add_constant(Value::I64(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::I64
                    }
                    crate::typechecker::ast::Literal::F32(value) => {
                        let index = constant_pool.add_constant(Value::F32(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::F32
                    }
                    crate::typechecker::ast::Literal::F64(value) => {
                        let index = constant_pool.add_constant(Value::F64(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::F64
                    }
                    crate::typechecker::ast::Literal::Char(value) => {
                        let index = constant_pool.add_constant(Value::Char(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::Char
                    }
                    crate::typechecker::ast::Literal::String(_) => {
                        let index = constant_pool.add_constant(Value::Str);
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::Str
                    }
                    crate::typechecker::ast::Literal::Bool(b) => {
                        let index = constant_pool.add_constant(Value::U8(if *b { 1 } else { 0 }));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U8
                    }
                }
            }
            Expression::Variable(name) => {
                self.lookup_local(name).into()
            }
        }
    }

}

impl IntoBinary for StatementsCompiler {
    fn into_binary(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.block_count as u64).to_le_bytes());
        bytes.extend_from_slice(&(self.bytecode.len() as u64).to_le_bytes());
        for bytecode in &self.bytecode {
            bytes.extend_from_slice(&bytecode.into_binary());
        }
        bytes
    }
}


impl IntoBinary for Bytecode {
    fn into_binary(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.into_instruction().to_le_bytes());
        match self {
            Bytecode::LoadConstant(index) => {
                bytes.extend_from_slice(&index.to_le_bytes());
            }
            Bytecode::StoreConstant(index) => {
                bytes.extend_from_slice(&index.to_le_bytes());
            }
            Bytecode::StoreLocal(index, ty) => {
                bytes.push(*index);
                bytes.push(*ty);
            }
            Bytecode::LoadLocal(index) => {
                bytes.push(*index);
            }
            Bytecode::Convert(tag) => {
                bytes.push(*tag);
            }
            Bytecode::BinaryConvert(tag) => {
                bytes.push(*tag);
            }
            Bytecode::Goto(blockid) => {
                bytes.extend_from_slice(&blockid.to_le_bytes());
            }
            Bytecode::If(blockid, elseid) => {
                bytes.extend_from_slice(&blockid.to_le_bytes());
                bytes.extend_from_slice(&elseid.to_le_bytes());
            }
            Bytecode::InvokeFunction(symbol) => {
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::InvokeFunctionTail(symbol) => {
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::InvokeTrait(symbol1, symbol2) => {
                bytes.extend_from_slice(&symbol1.to_le_bytes());
                bytes.extend_from_slice(&symbol2.to_le_bytes());
            }
            Bytecode::InvokeTraitTail(symbol1, symbol2) => {
                bytes.extend_from_slice(&symbol1.to_le_bytes());
                bytes.extend_from_slice(&symbol2.to_le_bytes());
            }
            Bytecode::CreateStruct(symbol) => {
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::CreateEnum(symbol) => {
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::IsA(symbol) => {
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::GetField(offset, tag) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
                bytes.push(*tag);
            }
            Bytecode::SetField(offset, tag) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
                bytes.push(*tag);
            }
            Bytecode::CreateArray(tag) => {
                bytes.push(*tag);
            }
            Bytecode::ArrayGet(tag) => {
                bytes.push(*tag);
            }
            Bytecode::ArraySet(tag) => {
                bytes.push(*tag);
            }
            Bytecode::StartBlock(block_id) => {
                bytes.extend_from_slice(&block_id.to_le_bytes());
            }
            _ => {}
        }
        bytes
    }
    
}
