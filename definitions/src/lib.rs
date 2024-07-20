use object::Reference;


pub mod class;
pub mod object;
pub mod bytecode;
pub mod stack;

pub enum ArgType {
    Unit,
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Reference(Reference),
}
    

pub type RustNativeMethod = fn(&[ArgType]) -> CocoaResult<ArgType>;

pub type CocoaResult<T> = Result<T, ErrorInfo>;

#[derive(Debug)]
pub struct ErrorInfo {
    message: String,
}
