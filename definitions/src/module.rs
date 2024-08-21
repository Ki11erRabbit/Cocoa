use crate::{FromBinary, IntoBinary};

use self::{constants::ConstantPool, enums::EnumTable, functions::FunctionTable, impls::TraitImplTable, structs::StructTable, traits::TraitTable};

pub mod constants;
pub mod functions;
pub mod structs;
pub mod enums;
pub mod traits;
pub mod impls;
pub mod locations;

pub type PoolIndex = u64;
pub type LocationIndex = u64;


pub struct Module {
    pub constant_pool: ConstantPool,
    pub function_table: FunctionTable,
    pub struct_table: StructTable,
    pub enum_table: EnumTable,
    pub trait_table: TraitTable,
    pub impl_table: TraitImplTable,
}


impl IntoBinary for Module {
    fn into_binary(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.constant_pool.into_binary());
        result.extend(self.function_table.into_binary());
        result.extend(self.struct_table.into_binary());
        //result.extend(self.enum_table.into_binary());
        //result.extend(self.trait_table.into_binary());
        //result.extend(self.impl_table.into_binary());
        result
    }
}

impl FromBinary for Module {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self {
        let constant_pool = ConstantPool::from_binary(source);
        let function_table = FunctionTable::from_binary(source);
        let struct_table = StructTable::from_binary(source);
        //let enum_table = EnumTable::from_binary(source);
        //let trait_table = TraitTable::from_binary(source);
        //let impl_table = TraitImplTable::from_binary(source);

        Module { constant_pool, function_table, struct_table, enum_table: EnumTable::default(), trait_table: TraitTable::default(), impl_table: TraitImplTable::default() }
    }
}


