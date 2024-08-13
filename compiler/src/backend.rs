
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
                bytes.extend_from_slice(&value.encode_utf8(&mut buffer).as_bytes());
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
        match self {
            Bytecode::LoadConstant(index) => {
                bytes.push(0);
                bytes.push(0);
                bytes.extend_from_slice(&index.to_le_bytes());
            }
            Bytecode::StoreConstant(index) => {
                bytes.push(1);
                bytes.push(0);
                bytes.extend_from_slice(&index.to_le_bytes());
            }
            Bytecode::Pop => {
                bytes.push(2);
                bytes.push(0);
            }
            Bytecode::Dup => {
                bytes.push(3);
                bytes.push(0);
            }
            Bytecode::Swap => {
                bytes.push(4);
                bytes.push(0);
            }
            Bytecode::StoreLocal(index) => {
                bytes.push(5);
                bytes.push(0);
                bytes.push(*index);
            }
            Bytecode::LoadLocal(index) => {
                bytes.push(6);
                bytes.push(0);
                bytes.push(*index);
            }
            Bytecode::StoreArgument => {
                bytes.push(7);
                bytes.push(0);
            }
            Bytecode::Addu8 => {
                bytes.push(8);
                bytes.push(0);
            }
            Bytecode::Addu16 => {
                bytes.push(9);
                bytes.push(0);
            }
            Bytecode::Addu32 => {
                bytes.push(10);
                bytes.push(0);
            }
            Bytecode::Addu64 => {
                bytes.push(11);
                bytes.push(0);
            }
            Bytecode::Addi8 => {
                bytes.push(12);
                bytes.push(0);
            }
            Bytecode::Addi16 => {
                bytes.push(13);
                bytes.push(0);
            }
            Bytecode::Addi32 => {
                bytes.push(14);
                bytes.push(0);
            }
            Bytecode::Addi64 => {
                bytes.push(15);
                bytes.push(0);
            }
            Bytecode::Subu8 => {
                bytes.push(16);
                bytes.push(0);
            }
            Bytecode::Subu16 => {
                bytes.push(17);
                bytes.push(0);
            }
            Bytecode::Subu32 => {
                bytes.push(18);
                bytes.push(0);
            }
            Bytecode::Subu64 => {
                bytes.push(19);
                bytes.push(0);
            }
            Bytecode::Subi8 => {
                bytes.push(20);
                bytes.push(0);
            }
            Bytecode::Subi16 => {
                bytes.push(21);
                bytes.push(0);
            }
            Bytecode::Subi32 => {
                bytes.push(22);
                bytes.push(0);
            }
            Bytecode::Subi64 => {
                bytes.push(23);
                bytes.push(0);
            }
            Bytecode::Mulu8 => {
                bytes.push(24);
                bytes.push(0);
            }
            Bytecode::Mulu16 => {
                bytes.push(25);
                bytes.push(0);
            }
            Bytecode::Mulu32 => {
                bytes.push(26);
                bytes.push(0);
            }
            Bytecode::Mulu64 => {
                bytes.push(27);
                bytes.push(0);
            }
            Bytecode::Muli8 => {
                bytes.push(28);
                bytes.push(0);
            }
            Bytecode::Muli16 => {
                bytes.push(29);
                bytes.push(0);
            }
            Bytecode::Muli32 => {
                bytes.push(30);
                bytes.push(0);
            }
            Bytecode::Muli64 => {
                bytes.push(31);
                bytes.push(0);
            }
            Bytecode::Divu8 => {
                bytes.push(32);
                bytes.push(0);
            }
            Bytecode::Divu16 => {
                bytes.push(33);
                bytes.push(0);
            }
            Bytecode::Divu32 => {
                bytes.push(34);
                bytes.push(0);
            }
            Bytecode::Divu64 => {
                bytes.push(35);
                bytes.push(0);
            }
            Bytecode::Divi8 => {
                bytes.push(36);
                bytes.push(0);
            }
            Bytecode::Divi16 => {
                bytes.push(37);
                bytes.push(0);
            }
            Bytecode::Divi32 => {
                bytes.push(38);
                bytes.push(0);
            }
            Bytecode::Divi64 => {
                bytes.push(39);
                bytes.push(0);
            }
            Bytecode::Modu8 => {
                bytes.push(40);
                bytes.push(0);
            }
            Bytecode::Modu16 => {
                bytes.push(41);
                bytes.push(0);
            }
            Bytecode::Modu32 => {
                bytes.push(42);
                bytes.push(0);
            }
            Bytecode::Modu64 => {
                bytes.push(43);
                bytes.push(0);
            }
            Bytecode::Modi8 => {
                bytes.push(44);
                bytes.push(0);
            }
            Bytecode::Modi16 => {
                bytes.push(45);
                bytes.push(0);
            }
            Bytecode::Modi32 => {
                bytes.push(46);
                bytes.push(0);
            }
            Bytecode::Modi64 => {
                bytes.push(47);
                bytes.push(0);
            }
            Bytecode::Andu8 => {
                bytes.push(48);
                bytes.push(0);
            }
            Bytecode::Andu16 => {
                bytes.push(49);
                bytes.push(0);
            }
            Bytecode::Andu32 => {
                bytes.push(50);
                bytes.push(0);
            }
            Bytecode::Andu64 => {
                bytes.push(51);
                bytes.push(0);
            }
            Bytecode::Andi8 => {
                bytes.push(52);
                bytes.push(0);
            }
            Bytecode::Andi16 => {
                bytes.push(53);
                bytes.push(0);
            }
            Bytecode::Andi32 => {
                bytes.push(54);
                bytes.push(0);
            }
            Bytecode::Andi64 => {
                bytes.push(55);
                bytes.push(0);
            }
            Bytecode::Oru8 => {
                bytes.push(56);
                bytes.push(0);
            }
            Bytecode::Oru16 => {
                bytes.push(57);
                bytes.push(0);
            }
            Bytecode::Oru32 => {
                bytes.push(58);
                bytes.push(0);
            }
            Bytecode::Oru64 => {
                bytes.push(59);
                bytes.push(0);
            }
            Bytecode::Ori8 => {
                bytes.push(60);
                bytes.push(0);
            }
            Bytecode::Ori16 => {
                bytes.push(61);
                bytes.push(0);
            }
            Bytecode::Ori32 => {
                bytes.push(62);
                bytes.push(0);
            }
            Bytecode::Ori64 => {
                bytes.push(63);
                bytes.push(0);
            }
            Bytecode::Xoru8 => {
                bytes.push(64);
                bytes.push(0);
            }
            Bytecode::Xoru16 => {
                bytes.push(65);
                bytes.push(0);
            }
            Bytecode::Xoru32 => {
                bytes.push(66);
                bytes.push(0);
            }
            Bytecode::Xoru64 => {
                bytes.push(67);
                bytes.push(0);
            }
            Bytecode::Xori8 => {
                bytes.push(68);
                bytes.push(0);
            }
            Bytecode::Xori16 => {
                bytes.push(69);
                bytes.push(0);
            }
            Bytecode::Xori32 => {
                bytes.push(70);
                bytes.push(0);
            }
            Bytecode::Xori64 => {
                bytes.push(71);
                bytes.push(0);
            }
            Bytecode::Notu8 => {
                bytes.push(72);
                bytes.push(0);
            }
            Bytecode::Notu16 => {
                bytes.push(73);
                bytes.push(0);
            }
            Bytecode::Notu32 => {
                bytes.push(74);
                bytes.push(0);
            }
            Bytecode::Notu64 => {
                bytes.push(75);
                bytes.push(0);
            }
            Bytecode::Noti8 => {
                bytes.push(76);
                bytes.push(0);
            }
            Bytecode::Noti16 => {
                bytes.push(77);
                bytes.push(0);
            }
            Bytecode::Noti32 => {
                bytes.push(78);
                bytes.push(0);
            }
            Bytecode::Noti64 => {
                bytes.push(79);
                bytes.push(0);
            }
            Bytecode::Shlu8 => {
                bytes.push(80);
                bytes.push(0);
            }
            Bytecode::Shlu16 => {
                bytes.push(81);
                bytes.push(0);
            }
            Bytecode::Shlu32 => {
                bytes.push(82);
                bytes.push(0);
            }
            Bytecode::Shlu64 => {
                bytes.push(83);
                bytes.push(0);
            }
            Bytecode::Shli8 => {
                bytes.push(84);
                bytes.push(0);
            }
            Bytecode::Shli16 => {
                bytes.push(85);
                bytes.push(0);
            }
            Bytecode::Shli32 => {
                bytes.push(86);
                bytes.push(0);
            }
            Bytecode::Shli64 => {
                bytes.push(87);
                bytes.push(0);
            }
            Bytecode::Shru8 => {
                bytes.push(88);
                bytes.push(0);
            }
            Bytecode::Shru16 => {
                bytes.push(89);
                bytes.push(0);
            }
            Bytecode::Shru32 => {
                bytes.push(90);
                bytes.push(0);
            }
            Bytecode::Shru64 => {
                bytes.push(91);
                bytes.push(0);
            }
            Bytecode::Shri8 => {
                bytes.push(92);
                bytes.push(0);
            }
            Bytecode::Shri16 => {
                bytes.push(93);
                bytes.push(0);
            }
            Bytecode::Shri32 => {
                bytes.push(94);
                bytes.push(0);
            }
            Bytecode::Shri64 => {
                bytes.push(95);
                bytes.push(0);
            }
            Bytecode::Addf32 => {
                bytes.push(96);
                bytes.push(0);
            }
            Bytecode::Addf64 => {
                bytes.push(97);
                bytes.push(0);
            }
            Bytecode::Subf32 => {
                bytes.push(98);
                bytes.push(0);
            }
            Bytecode::Subf64 => {
                bytes.push(99);
                bytes.push(0);
            }
            Bytecode::Mulf32 => {
                bytes.push(100);
                bytes.push(0);
            }
            Bytecode::Mulf64 => {
                bytes.push(101);
                bytes.push(0);
            }
            Bytecode::Divf32 => {
                bytes.push(102);
                bytes.push(0);
            }
            Bytecode::Divf64 => {
                bytes.push(103);
                bytes.push(0);
            }
            Bytecode::Modf32 => {
                bytes.push(104);
                bytes.push(0);
            }
            Bytecode::Modf64 => {
                bytes.push(105);
                bytes.push(0);
            }
            Bytecode::Negu8 => {
                bytes.push(106);
                bytes.push(0);
            }
            Bytecode::Negu16 => {
                bytes.push(107);
                bytes.push(0);
            }
            Bytecode::Negu32 => {
                bytes.push(108);
                bytes.push(0);
            }
            Bytecode::Negu64 => {
                bytes.push(109);
                bytes.push(0);
            }
            Bytecode::Negi8 => {
                bytes.push(110);
                bytes.push(0);
            }
            Bytecode::Negi16 => {
                bytes.push(111);
                bytes.push(0);
            }
            Bytecode::Negi32 => {
                bytes.push(112);
                bytes.push(0);
            }
            Bytecode::Negi64 => {
                bytes.push(113);
                bytes.push(0);
            }
            Bytecode::Equalu8 => {
                bytes.push(114);
                bytes.push(0);
            }
            Bytecode::Equalu16 => {
                bytes.push(115);
                bytes.push(0);
            }
            Bytecode::Equalu32 => {
                bytes.push(116);
                bytes.push(0);
            }
            Bytecode::Equalu64 => {
                bytes.push(117);
                bytes.push(0);
            }
            Bytecode::Equali8 => {
                bytes.push(118);
                bytes.push(0);
            }
            Bytecode::Equali16 => {
                bytes.push(119);
                bytes.push(0);
            }
            Bytecode::Equali32 => {
                bytes.push(120);
                bytes.push(0);
            }
            Bytecode::Equali64 => {
                bytes.push(121);
                bytes.push(0);
            }
            Bytecode::Equalf32 => {
                bytes.push(122);
                bytes.push(0);
            }
            Bytecode::Equalf64 => {
                bytes.push(123);
                bytes.push(0);
            }
            Bytecode::Greateru8 => {
                bytes.push(124);
                bytes.push(0);
            }
            Bytecode::Greateru16 => {
                bytes.push(125);
                bytes.push(0);
            }
            Bytecode::Greateru32 => {
                bytes.push(125);
                bytes.push(0);
            }
            Bytecode::Greateru64 => {
                bytes.push(126);
                bytes.push(0);
            }
            Bytecode::Greateri8 => {
                bytes.push(127);
                bytes.push(0);
            }
            Bytecode::Greateri16 => {
                bytes.push(128);
                bytes.push(0);
            }
            Bytecode::Greateri32 => {
                bytes.push(129);
                bytes.push(0);
            }
            Bytecode::Greateri64 => {
                bytes.push(130);
                bytes.push(0);
            }
            Bytecode::Greaterf32 => {
                bytes.push(131);
                bytes.push(0);
            }
            Bytecode::Greaterf64 => {
                bytes.push(132);
                bytes.push(0);
            }
            Bytecode::Lessu8 => {
                bytes.push(133);
                bytes.push(0);
            }
            Bytecode::Lessu16 => {
                bytes.push(134);
                bytes.push(0);
            }
            Bytecode::Lessu32 => {
                bytes.push(135);
                bytes.push(0);
            }
            Bytecode::Lessu64 => {
                bytes.push(136);
                bytes.push(0);
            }
            Bytecode::Lessi8 => {
                bytes.push(137);
                bytes.push(0);
            }
            Bytecode::Lessi16 => {
                bytes.push(138);
                bytes.push(0);
            }
            Bytecode::Lessi32 => {
                bytes.push(139);
                bytes.push(0);
            }
            Bytecode::Lessi64 => {
                bytes.push(140);
                bytes.push(0);
            }
            Bytecode::Lessf32 => {
                bytes.push(141);
                bytes.push(0);
            }
            Bytecode::Lessf64 => {
                bytes.push(142);
                bytes.push(0);
            }
            Bytecode::Convert(tag) => {
                bytes.push(143);
                bytes.push(0);
                bytes.push(*tag);
            }
            Bytecode::BinaryConvert(tag) => {
                bytes.push(144);
                bytes.push(0);
                bytes.push(*tag);
            }
            Bytecode::Goto(offset) => {
                bytes.push(145);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::Jump => {
                bytes.push(146);
                bytes.push(0);
            }
            Bytecode::If(offset) => {
                bytes.push(147);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfNot(offset) => {
                bytes.push(148);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfGreater(offset) => {
                bytes.push(149);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfGreaterEqual(offset) => {
                bytes.push(150);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfLess(offset) => {
                bytes.push(151);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfLessEqual(offset) => {
                bytes.push(152);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfNull(offset) => {
                bytes.push(153);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::IfNotNull(offset) => {
                bytes.push(154);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
            }
            Bytecode::InvokeFunction(symbol) => {
                bytes.push(155);
                bytes.push(0);
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::InvokeFunctionTail(symbol) => {
                bytes.push(156);
                bytes.push(0);
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::InvokeTrait(symbol1, symbol2) => {
                bytes.push(157);
                bytes.push(0);
                bytes.extend_from_slice(&symbol1.to_le_bytes());
                bytes.extend_from_slice(&symbol2.to_le_bytes());
            }
            Bytecode::InvokeTraitTail(symbol1, symbol2) => {
                bytes.push(158);
                bytes.push(0);
                bytes.extend_from_slice(&symbol1.to_le_bytes());
                bytes.extend_from_slice(&symbol2.to_le_bytes());
            }
            Bytecode::Return => {
                bytes.push(159);
                bytes.push(0);
            }
            Bytecode::ReturnUnit => {
                bytes.push(160);
                bytes.push(0);
            }
            Bytecode::CreateStruct(symbol) => {
                bytes.push(161);
                bytes.push(0);
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::CreateEnum(symbol) => {
                bytes.push(162);
                bytes.push(0);
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::IsA(symbol) => {
                bytes.push(163);
                bytes.push(0);
                bytes.extend_from_slice(&symbol.to_le_bytes());
            }
            Bytecode::GetField(offset, tag) => {
                bytes.push(164);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
                bytes.push(*tag);
            }
            Bytecode::SetField(offset, tag) => {
                bytes.push(165);
                bytes.push(0);
                bytes.extend_from_slice(&offset.to_le_bytes());
                bytes.push(*tag);
            }
            Bytecode::CreateArray(tag) => {
                bytes.push(166);
                bytes.push(0);
                bytes.push(*tag);
            }
            Bytecode::ArrayGet(tag) => {
                bytes.push(167);
                bytes.push(0);
                bytes.push(*tag);
            }
            Bytecode::ArraySet(tag) => {
                bytes.push(168);
                bytes.push(0);
                bytes.push(*tag);
            }
            Bytecode::Breakpoint => {
                bytes.push(169);
                bytes.push(0);
            }
            Bytecode::Nop => {
                bytes.push(170);
                bytes.push(0);
            }
        }

        bytes
    }
    
}
