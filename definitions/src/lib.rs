
pub mod class;
pub mod object;
pub mod bytecode;
pub mod stack;



pub type RustNativeMethod = fn(&[object::Object]) -> CocoaResult<object::Object>;

pub type CocoaResult<T> = Result<T, ErrorInfo>;

pub struct ErrorInfo {

}
