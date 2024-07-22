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
    /// Remove the top value from the stack
    Pop,
    /// Pushes a null value onto the stack. This is the size of a reference so a 64-bit value
    PushNull,
    /// Pushes a constant value onto the stack
    /// The constant comes from the current stack frame's class's constant pool
    LoadConstant(PoolIndex),
    /// Duplicates the top value on the stack
    Dup,
    /// Swaps the top two values on the stack
    Swap,
    // Local Variables
    /// Store the top value on the stack in a local variable
    /// There are 256 local variables available
    StoreLocal(u8),
    /// Load a local variable onto the stack
    LoadLocal(u8),
    // Arithmetic
    /// Add the top two values on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The values are only added if they are the same type, otherwise an error is thrown
    Add,
    /// Subtract the top two values on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The values are only subtracted if they are the same type, otherwise an error is thrown
    Subtract,
    /// Multiply the top two values on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The values are only multiplied if they are the same type, otherwise an error is thrown
    Multiply,
    /// Divide the top two values on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The values are only divided if they are the same type, otherwise an error is thrown
    Divide,
    /// Modulo the top two values on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The values are only moduloed if they are the same type, otherwise an error is thrown
    Modulo,
    /// Negate the top value on the stack
    /// The value is popped off the stack and the result is pushed back on
    /// The value is only negated if it is a number, otherwise an error is thrown
    /// There is also an error if the value was unsigned and smaller/larger than its signed counterpart
    Negate,
    // Bitwise
    /// Bitwise And the top two values on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The values are only bitwise anded if they are the same type, otherwise an error is thrown
    And,
    /// Bitwise Or the top two values on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The values are only bitwise ored if they are the same type, otherwise an error is thrown
    Or,
    /// Bitwise Xor the top two values on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The values are only bitwise xored if they are the same type, otherwise an error is thrown
    Xor,
    /// Bitwise Not the top value on the stack
    /// The value is popped off the stack and the result is pushed back on
    /// The value is only bitwise notted if it is an integer, otherwise an error is thrown
    Not,
    /// Shift the top value on the stack left by the second value on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The amount to be shifted is always an u32
    ShiftLeft,
    /// Shift the top value on the stack right by the second value on the stack
    /// The values are popped off the stack and the result is pushed back on
    /// The amount to be shifted is always an u32
    ShiftRight,
    // Comparison
    /// Compare the top two values on the stack for equality
    /// The values are popped off the stack and the result is pushed back on
    /// The output is a i8 where 0 is equal and anything else is not equal
    /// The values are only compared if they are the same type, otherwise an error is thrown
    Equal,
    /// Compare the top two values on the stack for inequality
    /// The values are popped off the stack and the result is pushed back on
    /// The output is a i8 where 0 is not equal and a 1 is greater than
    /// The values are only compared if they are the same type, otherwise an error is thrown
    Greater,
    /// Compare the top two values on the stack for inequality
    /// The values are popped off the stack and the result is pushed back on
    /// The output is a i8 where 0 is not equal and a -1 is less than
    /// The values are only compared if they are the same type, otherwise an error is thrown
    Less,
    // Conversion
    /// Convert the top value on the stack to a different type
    /// The value is popped off the stack and the result is pushed back on
    /// The value is only converted if it is a number, otherwise an error is thrown
    /// The value is coerced the type specified by Rust as syntax
    Convert(Type),
    /// Convert the top value on the stack via a binary representation
    /// The value is popped off the stack and the result is pushed back on
    /// This converts the raw binary data of the value to the type specified
    /// This only works for types of the same size
    BinaryConvert(Type),
    // Control Flow
    /// Jump to an offset in the bytecode
    Goto(Offset),
    /// Jump to an offset in the bytecode if the top value on the stack is equal (i.e. 0)
    If(Offset),
    /// Jump to an offset in the bytecode if the top value on the stack is not equal (i.e. 1)
    IfNot(Offset),
    /// Jump to an offset in the bytecode if the top value on the stack is greater than (i.e. 1)
    IfGreater(Offset),
    /// Jump to an offset in the bytecode if the top value on the stack is greater than or equal (i.e. 0 or 1)
    IfGreaterEqual(Offset),
    /// Jump to an offset in the bytecode if the top value on the stack is less than (i.e. -1)
    IfLess(Offset),
    /// Jump to an offset in the bytecode if the top value on the stack is less than or equal (i.e. 0 or -1)
    IfLessEqual(Offset),
    /// Jump to an offset in the bytecode if the top value on the stack is null
    IfNull(Offset),
    /// Jump to an offset in the bytecode if the top value on the stack is not null
    IfNotNull(Offset),
    /// Invoke a method on the current object
    /// This will call a parent method if the method is not found in the current class
    InvokeVirtual(MethodIndex),
    /// Invoke a static method
    /// This will call a method in the current class
    /// TODO: This should be able to call a method from another class
    InvokeStatic(MethodIndex),
    /// Invoke a method on an interface
    /// PoolIndex is the info of the interface found in the constant pool of the class
    /// MethodIndex is the index of the method in the interface info struct
    InvokeInterface(PoolIndex, MethodIndex),
    /// Return from the current method
    /// This pops the top value off the stack and returns it onto the stack of the calling method
    Return,
    // Object Related
    /// Create a new object
    /// The PoolIndex is the class info of the object to be created
    New(PoolIndex),
    /// Set a field in the current object
    /// The FieldIndex is the index of the field in the class's field table
    SetField(FieldIndex),
    /// Get a field from the current object
    /// The FieldIndex is the index of the field in the class's field table
    GetField(FieldIndex),
    /// Load a static field
    /// The FieldIndex is the index of the field in the class's field table
    StoreStatic(FieldIndex),
    /// Store a static field
    /// The FieldIndex is the index of the field in the class's field table
    LoadStatic(FieldIndex),
    /// Check if the top value on the stack is an instance of the class
    /// The PoolIndex is the class info of the class to check against
    InstanceOf(PoolIndex),
    // Array Related
    /// Create a new array with a specified type
    NewArray(Type),
    /// Get an element from an array
    /// The index is popped off the stack and the element is pushed back on
    ArrayGet(Type),
    /// Set an element in an array
    /// The index and value are popped off the stack
    ArraySet(Type),
    // String Related
    /// Create a new string
    /// The StringIndex is the index of the string in the class's string table
    NewString(StringIndex),
    // Misc
    /// Breakpoint for debugging
    Breakpoint,
    /// No operation
    Nop,
}
