mod object_table;
mod machine;
mod constant_pool;
mod linker;

use definitions::ArgType;
pub use object_table::ObjectTableSingleton;
pub use machine::Machine;
pub use machine::MethodTable;
pub use machine::ObjectTable;
pub use machine::ConstantPool;
pub use linker::Linker;
pub use constant_pool::ConstantPoolSingleton;

use definitions::CocoaResult;
use once_cell::sync::Lazy;

pub type RustNativeMethod = fn(
    &[ArgType],
    object_table: &dyn ObjectTable,
    method_table: &dyn MethodTable,
    constant_pool: &dyn ConstantPool
) -> CocoaResult<ArgType>;

#[derive(Clone, Copy)]
enum NativeMethod {
    Rust(RustNativeMethod),
}

static NATIVE_METHOD_TABLE: Lazy<Vec<NativeMethod>> = Lazy::new(|| {
    vec![
        NativeMethod::Rust(array_size),
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

fn array_size(
    args: &[ArgType],
    object_table: &dyn ObjectTable,
    _: &dyn MethodTable,
    _: &dyn ConstantPool
) -> CocoaResult<ArgType> {
    if let ArgType::Reference(reference) = args[0] {
        if !object_table.is_array(reference) {
            todo!("Handle non-array reference")
        }
        let object = object_table.get_array(reference);
        Ok(ArgType::U64(object.get_size() as u64))
    } else {
        todo!("Handle non-reference argument")
    }
}
