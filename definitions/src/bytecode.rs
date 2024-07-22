use crate::class::PoolIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
pub type MethodIndex = usize;
pub type FieldIndex = usize;
pub type StringIndex = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    Greater,
    Less,
    // Conversion
    Convert(Type),
    BinaryConvert(Type),
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
    InvokeVirtual(MethodIndex),
    InvokeStatic(MethodIndex),
    /// Invoke a method on an interface
    /// PoolIndex is the info of the interface found in the constant pool of the class
    /// MethodIndex is the index of the method in the interface info struct
    InvokeInterface(PoolIndex, MethodIndex),
    Return,
    // Object Related
    New(PoolIndex),
    SetField(FieldIndex),
    GetField(FieldIndex),
    StoreStatic(FieldIndex),
    LoadStatic(FieldIndex),
    InstanceOf(PoolIndex),
    // Array Related
    NewArray(Type, usize),
    ArrayGet(Type),
    ArraySet(Type),
    // String Related
    NewString(StringIndex),
    // Misc
    Breakpoint,
    Nop,
}
