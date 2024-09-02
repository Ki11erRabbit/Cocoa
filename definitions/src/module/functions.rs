use crate::{bytecode::Bytecode, FromBinary, IntoBinary};

use super::{LocationIndex, PoolIndex};




pub struct Function {
    pub name_symbol: PoolIndex,
    pub symbol_name: PoolIndex,
    pub location_index: LocationIndex,
    pub flags: FunctionFlags,
    pub block_count: u64,
    pub byte_code: Vec<Bytecode>,
}

bitflags::bitflags! {
    pub struct FunctionFlags: u8 {
        const Public = 0b00000001;
        const Private = 0b00000010;
        const PackagePrivate = 0b00000100;
    }
}


impl IntoBinary for Function {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.name_symbol.to_le_bytes());
        result.extend(self.symbol_name.to_le_bytes());
        result.extend(self.location_index.to_le_bytes());
        result.push(self.flags.bits());
        let size = self.byte_code.len() as u64;
        result.extend(size.to_le_bytes().iter());
        result.extend(self.block_count.to_le_bytes().iter());
        result.extend(self.byte_code.iter().map(|x| x.into_binary()).flatten());
        result
    }
}

impl FromBinary for Function {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        let name_symbol = PoolIndex::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let symbol_name = PoolIndex::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let location_index = LocationIndex::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let flags = FunctionFlags::from_bits_truncate(source.next().unwrap());
        let size = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let block_count = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let byte_code = (0..size).map(|_| Bytecode::from_binary(source)).collect();

        Function { name_symbol, symbol_name, location_index, flags, block_count, byte_code }
    }
}

pub struct FunctionTable {
    pub functions: Vec<Function>,
}

impl FunctionTable {
    pub fn new() -> Self {
        FunctionTable { functions: Vec::new() }
    }

    pub fn iter(&self) -> std::slice::Iter<Function> {
        self.functions.iter()
    }
}

impl IntoBinary for FunctionTable {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let size = self.functions.len() as u64;
        result.extend(size.to_le_bytes().iter());
        result.extend(self.functions.iter().map(|x| x.into_binary()).flatten());
        result
    }
}

impl FromBinary for FunctionTable {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        let size = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let functions = (0..size).map(|_| Function::from_binary(source)).collect();

        FunctionTable { functions }
    }
}
