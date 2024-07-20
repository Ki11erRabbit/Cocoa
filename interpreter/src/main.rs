use definitions::{bytecode::Bytecode, class::{ClassHeader, ClassInfo, Method, MethodFlags, MethodInfo, PoolEntry}};
use virtual_machine::{Machine, NativeMethodTable, ObjectTableSingleton};
use virtual_machine::ObjectTable;

mod virtual_machine;


fn main() {

    let mut class = ClassHeader::new(6, 0, 0, 2);
    
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
    class.set_constant_pool_entry(2, PoolEntry::String("Main"));
    class.set_constant_pool_entry(3, PoolEntry::String("Object"));
    class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::InvokeStatic(5), Bytecode::Return].into(),0)));
    class.set_constant_pool_entry(5, PoolEntry::Method(Method::Native(0, 1)));

    class.set_method(0, MethodInfo {
        flags: MethodFlags::Static,
        name: 0,
        type_info: 0,
        location: 4,
    });

    class.set_method(1, MethodInfo {
        flags: MethodFlags::Static,
        name: 0,
        type_info: 0,
        location: 5,
    });


    let object_table = ObjectTableSingleton::get_singleton();

    let class_ref = object_table.add_class(class);
    let method_table = NativeMethodTable::get_table();

    let mut vm = Machine::new(&object_table, &method_table);

    vm.run_bootstrap(class_ref, 0).unwrap();
}
