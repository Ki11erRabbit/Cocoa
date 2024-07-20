mod object_table;
mod machine;

use definitions::ArgType;
pub use object_table::ObjectTableSingleton;
pub use machine::Machine;
pub use machine::MethodTable;
pub use machine::ObjectTable;

use definitions::{CocoaResult, RustNativeMethod};
use once_cell::sync::Lazy;


#[derive(Clone, Copy)]
enum NativeMethod {
    Rust(RustNativeMethod),
}

static NATIVE_METHOD_TABLE: Lazy<Vec<NativeMethod>> = Lazy::new(|| {
    vec![
        NativeMethod::Rust(hello_world),
]});

pub struct NativeMethodTable {}

impl NativeMethodTable {
    pub fn get_table() -> Self {
        Self {}
    }

}

impl MethodTable for NativeMethodTable {
    fn get_method(&self, index: usize) -> NativeMethod {
        NATIVE_METHOD_TABLE[index]
    }
}


fn hello_world(_: &[ArgType]) -> CocoaResult<ArgType> {
    println!("Hello, world!");
    Ok(ArgType::U64(0))
}
