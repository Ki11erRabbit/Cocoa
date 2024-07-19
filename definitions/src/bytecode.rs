use crate::class::PoolIndex;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Char(u8),
    Reference,
}

pub type Offset = isize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bytecode {
    // Stack manipulation
    Pop,
    PushNull,
    LoadConstant(PoolIndex),
    Dup,
    Swap,
    // Local Variables
    StoreLocal(u8),
    LoadLocal(u8),
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Negate,
    // Bitwise
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    ShiftRight,
    // Comparison
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Conversion
    Convert(Type, Type),
    // Control Flow
    Goto(Offset),
    If(Offset),
    IfNot(Offset),
    IfGreater(Offset),
    IfGreaterEqual(Offset),
    IfLess(Offset),
    IfLessEqual(Offset),
    IfNull(Offset),
    IfNotNull(Offset),
    InvokeVirtual(PoolIndex),
    InvokeStatic(PoolIndex),
    InvokeInterface(PoolIndex),
    Return,
    // Object Related
    New(PoolIndex),
    GetParent,
    SetField(PoolIndex),
    GetField(PoolIndex),
    StoreStatic(PoolIndex),
    LoadStatic(PoolIndex),
    // Array Related
    NewArray(Type, usize),
    ArrayGet(Type, usize),
    ArraySet(Type, usize),
    // Misc
    Breakpoint,
    Nop,
    InstanceOf(PoolIndex),
}
