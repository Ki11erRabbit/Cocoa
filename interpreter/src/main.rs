use definitions::{bytecode::Bytecode, class::{ClassHeader, ClassInfo, Method, MethodFlags, MethodInfo, PoolEntry, TypeInfo}};
use virtual_machine::{ConstantPoolSingleton, Linker, Machine, NativeMethodTable, ObjectTableSingleton};
use virtual_machine::ObjectTable;

mod virtual_machine;


fn main() {

    let mut class = ClassHeader::new(8, 0, 0, 2);
    
    class.set_parent_info(1);
    class.set_this_info(0);

    class.set_constant_pool_entry(0, PoolEntry::ClassInfo(ClassInfo {
        name: 2,
        class_ref: None,
    }));
    class.set_constant_pool_entry(1, PoolEntry::ClassInfo(ClassInfo {
        name: 3,
        class_ref: None,
    }));
    class.set_constant_pool_entry(2, PoolEntry::String("Main".to_owned()));
    class.set_constant_pool_entry(3, PoolEntry::String("Object".to_owned()));
    class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::InvokeStatic(1), Bytecode::Return].into())));
    class.set_constant_pool_entry(5, PoolEntry::Method(Method::Native(0)));
    class.set_constant_pool_entry(6, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));
    class.set_constant_pool_entry(7, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));

    class.set_method(0, MethodInfo {
        flags: MethodFlags::Static,
        name: 2,
        type_info: 6,
        location: 4,
    });

    class.set_method(1, MethodInfo {
        flags: MethodFlags::Static,
        name: 2,
        type_info: 7,
        location: 5,
    });

    let constant_pool = ConstantPoolSingleton::new();

    let mut linker = Linker::new(&constant_pool);

    linker.link_classes(&mut [class]);


    let object_table = ObjectTableSingleton::get_singleton();

    let class_ref = object_table.add_class(class);
    let method_table = NativeMethodTable::get_table();

    let mut vm = Machine::new(&object_table, &method_table, &constant_pool);

    vm.run_bootstrap(class_ref, 0).unwrap();
}
