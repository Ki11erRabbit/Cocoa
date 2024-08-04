
use std::sync::Arc;

use crate::{bytecode::{Bytecode, MethodIndex}, object::Reference};



pub type PoolIndex = usize;
pub type NativeMethodIndex = usize;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClassFlags: u8 {
        const Public = 0x01;
        const Private = 0x02;
        const Protected = 0x04;
        const Abstract = 0x08;
        const Interface = 0x10;
        const Static = 0x20;
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PoolEntry {
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
    ClassInfo(ClassInfo),
    Method(Method),
    TypeInfo(TypeInfo),
    Redirect(PoolIndex),
    Reference(Reference),
    Blank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassInfo {
    pub name: PoolIndex,
    pub class_ref: Option<Reference>
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    Native(NativeMethodIndex),
    Bytecode(Arc<[Bytecode]>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeInfo {
    Unit,
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
    Bool,
    String,
    Array(Box<TypeInfo>),
    Object(PoolIndex),
    Method {
        args: Vec<PoolIndex>,
        ret: Box<PoolIndex>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldInfo {
    pub name: PoolIndex,
    pub flags: FieldFlags,
    pub type_info: PoolIndex,
    /// The location of the field's value in the object
    pub location: Option<PoolIndex>,
}

impl FieldInfo {
    pub fn is_static(&self) -> bool {
        self.flags.contains(FieldFlags::Static)
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FieldFlags: u8 {
        const Public = 0x01;
        const Private = 0x02;
        const Protected = 0x04;
        const Static = 0x08;
        const Const = 0x10;
        const Synthetic = 0x20;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MethodInfo {
    pub flags: MethodFlags,
    pub name: PoolIndex,
    pub type_info: PoolIndex,
    pub location: PoolIndex,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MethodFlags: u8 {
        const Public = 0x01;
        const Private = 0x02;
        const Protected = 0x04;
        const Static = 0x08;
        const Const = 0x10;
        const Abstract = 0x20;
        const VaArgs = 0x40;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterfaceInfo {
    pub info: PoolIndex,
    pub vtable: Vec<MethodIndex>,
}

impl Default for InterfaceInfo {
    fn default() -> Self {
        InterfaceInfo {
            info: 0,
            vtable: Vec::new(),
        }
    }
}

pub struct ClassHeader {
    pub this_info: PoolIndex,
    pub parent_info: PoolIndex,
    pub class_flags: ClassFlags,
    pub constant_pool: Vec<PoolEntry>,
    pub interfaces: Vec<InterfaceInfo>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub strings: Vec<PoolIndex>,
}

impl ClassHeader {
    pub fn new(constant_pool_size: usize, interfaces_count: usize, fields_count: usize, methods_count: usize, string_count: usize) -> Self {
        let mut constant_pool = Vec::with_capacity(constant_pool_size);
        constant_pool.resize_with(constant_pool_size, || PoolEntry::U8(0));
        let mut interfaces = Vec::with_capacity(interfaces_count);
        interfaces.resize_with(interfaces_count, || InterfaceInfo::default());
        let mut fields = Vec::with_capacity(fields_count);
        fields.resize_with(fields_count, || FieldInfo {
            name: 0,
            flags: FieldFlags::empty(),
            type_info: 0,
            location: None,
        });
        let mut methods = Vec::with_capacity(methods_count);
        methods.resize_with(methods_count, || MethodInfo {
            flags: MethodFlags::empty(),
            name: 0,
            type_info: 0,
            location: 0,
        });

        let mut strings = Vec::with_capacity(string_count);
        strings.resize_with(string_count, || 0);
        
        ClassHeader {
            this_info: 0,
            parent_info: 0,
            class_flags: ClassFlags::empty(),
            constant_pool,
            interfaces,
            fields,
            methods,
            strings,
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::class::FieldFlags;
    use crate::class::{FieldInfo, MethodInfo, PoolEntry, MethodFlags};
    use super::*;

    #[test]
    fn test_class_header_creation() {
        let _ = ClassHeader::new(10, 5, 3, 4, 0);

    }

    
}
