use crate::{FromBinary, IntoBinary};


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Constant {
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
    String(String),
    TypeInfo(TypeInfo),
    Symbol(Symbol),
}

impl IntoBinary for Constant {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        use Constant::*;
        match self {
            U8(value) => {
                result.push(0);
                result.extend(value.to_le_bytes().iter());
            }
            U16(value) => {
                result.push(1);
                result.extend(value.to_le_bytes().iter());
            }
            U32(value) => {
                result.push(2);
                result.extend(value.to_le_bytes().iter());
            }
            U64(value) => {
                result.push(3);
                result.extend(value.to_le_bytes().iter());
            }
            I8(value) => {
                result.push(4);
                result.extend(value.to_le_bytes().iter());
            }
            I16(value) => {
                result.push(5);
                result.extend(value.to_le_bytes().iter());
            }
            I32(value) => {
                result.push(6);
                result.extend(value.to_le_bytes().iter());
            }
            I64(value) => {
                result.push(7);
                result.extend(value.to_le_bytes().iter());
            }
            F32(value) => {
                result.push(8);
                result.extend(value.to_le_bytes().iter());
            }
            F64(value) => {
                result.push(9);
                result.extend(value.to_le_bytes().iter());
            }
            Char(value) => {
                result.push(10);
                result.extend((*value as u32).to_le_bytes().iter());
            }
            String(value) => {
                result.push(11);
                result.extend(value.len().to_le_bytes().iter());
                result.extend(value.as_bytes());
            }
            TypeInfo(value) => {
                result.push(12);
                result.extend(value.into_binary());
            }
            Symbol(value) => {
                result.push(13);
                result.extend(value.into_binary());
            }
        }
        result
    }
}

impl FromBinary for Constant {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        match source.next().unwrap() {
            0 => Constant::U8(u8::from_le_bytes([source.next().unwrap()])),
            1 => Constant::U16(u16::from_le_bytes([source.next().unwrap(), source.next().unwrap()])),
            2 => Constant::U32(u32::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()])),
            2 => Constant::U64(u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()])),
            4 => Constant::I8(i8::from_le_bytes([source.next().unwrap()])),
            5 => Constant::I16(i16::from_le_bytes([source.next().unwrap(), source.next().unwrap()])),
            6 => Constant::I32(i32::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()])),
            7 => Constant::I64(i64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()])),
            8 => Constant::F32(f32::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()])),
            9 => Constant::F64(f64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()])),
            10 => Constant::Char(char::from_u32(u32::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()])).unwrap()),
            11 => {
                let length = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
                let value = String::from_utf8(source.take(length as usize).collect()).unwrap();
                Constant::String(value)
            }
            12 => Constant::TypeInfo(TypeInfo::from_binary(source)),
            13 => Constant::Symbol(Symbol::from_binary(source)),
            _ => panic!("Invalid constant type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Symbol {
    pub name: String,
    pub type_info: TypeInfo,
}

impl IntoBinary for Symbol {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.name.len().to_le_bytes().iter());
        result.extend(self.name.as_bytes());
        result.extend(self.type_info.into_binary());
        result
    }
}

impl FromBinary for Symbol {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        let length = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let name = String::from_utf8(source.take(length as usize).collect()).unwrap();
        let type_info = TypeInfo::from_binary(source);
        Symbol { name, type_info }
    }
}
            


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TypeInfo {
    Simple(Type),
    Function(Vec<TypeInfo>, Box<TypeInfo>),
}

impl IntoBinary for TypeInfo {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        use TypeInfo::*;
        match self {
            Simple(value) => {
                result.push(0);
                result.extend(value.into_binary());
            }
            Function(params, return_type) => {
                result.push(1);
                let size = params.len() as u64;
                result.extend(size.to_le_bytes().iter());
                result.extend(params.iter().map(|x| x.into_binary()).flatten());
                result.extend(return_type.into_binary());
            }
        }
        result
    }
}

impl FromBinary for TypeInfo {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        match source.next().unwrap() {
            0 => TypeInfo::Simple(Type::from_binary(source)),
            1 => {
                let size = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
                let params = (0..size).map(|_| TypeInfo::from_binary(source)).collect();
                let return_type = Box::new(TypeInfo::from_binary(source));
                TypeInfo::Function(params, return_type)
            }
            _ => panic!("Invalid type info type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    String,
}

impl IntoBinary for Type {
    fn into_binary(&self) -> Vec<u8> {
        vec![match self {
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
            Type::String => 11,
        }]
    }
}

impl FromBinary for Type {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        match source.next().unwrap() {
            0 => Type::U8,
            1 => Type::U16,
            2 => Type::U32,
            3 => Type::U64,
            4 => Type::I8,
            5 => Type::I16,
            6 => Type::I32,
            7 => Type::I64,
            8 => Type::F32,
            9 => Type::F64,
            10 => Type::Char,
            11 => Type::String,
            _ => panic!("Invalid type"),
        }
    }
}

pub struct ConstantPool {
    pub constants: Vec<Constant>,
}


impl IntoBinary for ConstantPool {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let size = self.constants.len() as u64;
        result.extend(size.to_le_bytes().iter());
        result.extend(self.constants.iter().map(|x| x.into_binary()).flatten());
        result
    }
}

impl FromBinary for ConstantPool {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        let size = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let constants = (0..size).map(|_| Constant::from_binary(source)).collect();
        ConstantPool { constants }
    }
}
