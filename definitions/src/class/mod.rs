pub mod regular;



pub type PoolIndex = usize;
pub type Object = *const ();
pub type NativeMethodIndex = usize;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClassFlags: u8 {
        const Public = 0x01;
        const Final = 0x02;
        const Super = 0x20;
        const Interface = 0x40;
        const Abstract = 0x80;
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PoolEntry<'a> {
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
    String(&'a str),
    ClassInfo(ClassInfo),
    Method(Method),
    TypeInfo(TypeInfo),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassInfo {
    name: PoolIndex,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    Native(NativeMethodIndex),
    //Bytecode(Box<[Bytecode]>),
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
        args: Vec<TypeInfo>,
        ret: Box<TypeInfo>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldInfo {
    name: PoolIndex,
    flags: FieldFlags,
    type_info: PoolIndex,
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
    flags: MethodFlags,
    name: PoolIndex,
    type_info: PoolIndex,
    location: PoolIndex,
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
