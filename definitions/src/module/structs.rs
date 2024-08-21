use crate::{FromBinary, IntoBinary};

use super::functions::Function;
use super::PoolIndex;


pub struct Struct {
    pub name_symbol: PoolIndex,
    pub symbol_name: PoolIndex,
    pub flags: StructFlags,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<Function>,
}


bitflags::bitflags! {
    pub struct StructFlags: u8 {
        const Public = 0b00000001;
        const Private = 0b00000010;
        const PackagePrivate = 0b00000100;
    }
}

impl IntoBinary for Struct {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.name_symbol.to_le_bytes());
        result.extend(self.symbol_name.to_le_bytes());
        result.push(self.flags.bits());
        let size = self.fields.len() as u64;
        result.extend(size.to_le_bytes().iter());
        result.extend(self.fields.iter().map(|x| x.into_binary()).flatten());
        let size = self.methods.len() as u64;
        result.extend(size.to_le_bytes().iter());
        result.extend(self.methods.iter().map(|x| x.into_binary()).flatten());
        result
    }
}

impl FromBinary for Struct {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        let name_symbol = PoolIndex::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let symbol_name = PoolIndex::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let flags = StructFlags::from_bits_truncate(source.next().unwrap());
        let size = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let fields = (0..size).map(|_| FieldInfo::from_binary(source)).collect();
        let size = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let methods = (0..size).map(|_| Function::from_binary(source)).collect();

        Struct { name_symbol, symbol_name, flags, fields, methods }
    }
}


pub struct FieldInfo {
    pub name_symbol: PoolIndex,
    pub flags: FieldFlags,
}

bitflags::bitflags! {
    pub struct FieldFlags: u8 {
        const Public = 0b00000001;
        const Private = 0b00000010;
        const PackagePrivate = 0b00000100;
    }
}

impl IntoBinary for FieldInfo {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.name_symbol.to_le_bytes());
        result.push(self.flags.bits());
        result
    }
}

impl FromBinary for FieldInfo {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        let name_symbol = PoolIndex::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let flags = FieldFlags::from_bits_truncate(source.next().unwrap());

        FieldInfo { name_symbol, flags }
    }
}

pub struct StructTable {
    pub structs: Vec<Struct>,
}

impl IntoBinary for StructTable {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let size = self.structs.len() as u64;
        result.extend(size.to_le_bytes().iter());
        result.extend(self.structs.iter().map(|x| x.into_binary()).flatten());
        result
    }
}

impl FromBinary for StructTable {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        let size = u64::from_le_bytes([source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()]);
        let structs = (0..size).map(|_| Struct::from_binary(source)).collect();

        StructTable { structs }
    }
}
