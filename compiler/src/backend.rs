
use bytecode::Bytecode;

use crate::ast::{BinaryOperator, Expression, PrefixOperator, SpannedExpression, SpannedStatement, Statement};


pub trait IntoBinary {
    fn into_binary(&self) -> Vec<u8>;
}

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
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            operands: Vec::new(),
            locals: [Value::U8(0); 256],
        }
    }

    pub fn push_value(&mut self, value: Value) {
        self.operands.push(value);
    }

    pub fn pop_value(&mut self) -> Value {
        self.operands.pop().unwrap()
    }
}

pub struct Stack {
    frames: Vec<Frame>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            frames: Vec::new(),
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
}


impl StatementsCompiler {
    pub fn new() -> Self {
        StatementsCompiler {
            stack: Stack::new(),
            bytecode: Vec::new(),
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
        }
    }

    fn compile_expression(&mut self, constant_pool: &mut ConstantPool, expr: &SpannedExpression) -> Type {
        match &expr.expression {
            Expression::BinaryExpression { left, operator, right } => {
                let ty1 = self.compile_expression(constant_pool, left);
                let ty2 = self.compile_expression(constant_pool, right);
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
                    crate::ast::Literal::U8(value) => {
                        let index = constant_pool.add_constant(Value::U8(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U8
                    }
                    crate::ast::Literal::U16(value) => {
                        let index = constant_pool.add_constant(Value::U16(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U16
                    }
                    crate::ast::Literal::U32(value) => {
                        let index = constant_pool.add_constant(Value::U32(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U32
                    }
                    crate::ast::Literal::U64(value) => {
                        let index = constant_pool.add_constant(Value::U64(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U64
                    }
                    crate::ast::Literal::I8(value) => {
                        let index = constant_pool.add_constant(Value::I8(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::I8
                    }
                    crate::ast::Literal::I16(value) => {
                        let index = constant_pool.add_constant(Value::I16(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::I16
                    }
                    crate::ast::Literal::I32(value) => {
                        let index = constant_pool.add_constant(Value::I32(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::I32
                    }
                    crate::ast::Literal::I64(value) => {
                        let index = constant_pool.add_constant(Value::I64(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::I64
                    }
                    crate::ast::Literal::Int(value) => {
                        let index = constant_pool.add_constant(Value::I64((*value) as i64));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::I64
                    }
                    crate::ast::Literal::F32(value) => {
                        let index = constant_pool.add_constant(Value::F32(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::F32
                    }
                    crate::ast::Literal::F64(value) => {
                        let index = constant_pool.add_constant(Value::F64(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::F64
                    }
                    crate::ast::Literal::Char(value) => {
                        let index = constant_pool.add_constant(Value::Char(*value));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::Char
                    }
                    crate::ast::Literal::String(_) => {
                        let index = constant_pool.add_constant(Value::Str);
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::Str
                    }
                    crate::ast::Literal::Bool(b) => {
                        let index = constant_pool.add_constant(Value::U8(if *b { 1 } else { 0 }));
                        self.bytecode.push(Bytecode::LoadConstant(index));
                        Type::U8
                    }
                }
            }
        }
    }

}

impl IntoBinary for StatementsCompiler {
    fn into_binary(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
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
            Bytecode::StoreLocal(index) => {
                bytes.push(*index);
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
            Bytecode::Goto(offset) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::If(offset) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfNot(offset) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfGreater(offset) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfGreaterEqual(offset) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfLess(offset) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfLessEqual(offset) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfNull(offset) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfNotNull(offset) => {
                bytes.extend_from_slice(&offset.to_le_bytes());
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
            _ => {}
        }
        bytes
    }
    
}
