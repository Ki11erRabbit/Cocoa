use definitions::{bytecode::Bytecode, class::{ClassHeader, ClassInfo, Method, MethodFlags, MethodInfo, PoolEntry, TypeInfo}};
use virtual_machine::{ConstantPoolSingleton, Linker, Machine, NativeMethodTable, ObjectTableSingleton};

mod virtual_machine;


fn main() {

        let mut class = ClassHeader::new(10, 0, 0, 2, 0);

        class.set_parent_info(1);
        class.set_this_info(0);

        class.set_constant_pool_entry(0, PoolEntry::ClassInfo(ClassInfo {
            name: 2,
            class_ref: Some(0),
        }));
        class.set_constant_pool_entry(1, PoolEntry::ClassInfo(ClassInfo {
            name: 3,
            class_ref: None,
        }));
        class.set_constant_pool_entry(2, PoolEntry::String("MainBase".to_owned()));
        class.set_constant_pool_entry(3, PoolEntry::String("Object".to_owned()));
        class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::New(0), Bytecode::InvokeVirtual(1), Bytecode::Return].into())));
        class.set_constant_pool_entry(5, PoolEntry::Method(Method::Native(0)));
        class.set_constant_pool_entry(6, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(7, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![TypeInfo::Object(3)], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(8, PoolEntry::String("printRef".to_owned()));

        class.set_method(0, MethodInfo {
            flags: MethodFlags::Static,
            name: 2,
            type_info: 6,
            location: 4,
        });

        class.set_method(1, MethodInfo {
            flags: MethodFlags::Public,
            name: 8,
            type_info: 7,
            location: 5,
        });
        
        let parent_class = class;
        
        let mut class = ClassHeader::new(10, 0, 0, 2, 0);

        class.set_parent_info(1);
        class.set_this_info(0);

        class.set_constant_pool_entry(0, PoolEntry::ClassInfo(ClassInfo {
            name: 2,
            class_ref: Some(0),
        }));
        class.set_constant_pool_entry(1, PoolEntry::ClassInfo(ClassInfo {
            name: 3,
            class_ref: None,
        }));
        class.set_constant_pool_entry(2, PoolEntry::String("Main".to_owned()));
        class.set_constant_pool_entry(3, PoolEntry::String("MainBase".to_owned()));
        class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::New(0), Bytecode::InvokeVirtual(1), Bytecode::Return].into())));
        class.set_constant_pool_entry(5, PoolEntry::Method(Method::Foreign(1)));
        class.set_constant_pool_entry(6, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(7, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![TypeInfo::Object(3)], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(8, PoolEntry::String("printRef".to_owned()));

        class.set_method(0, MethodInfo {
            flags: MethodFlags::Static,
            name: 2,
            type_info: 6,
            location: 4,
        });

        class.set_method(1, MethodInfo {
            flags: MethodFlags::Public,
            name: 8,
            type_info: 7,
            location: 5,
        });

        let constant_pool = ConstantPoolSingleton::new();
        let object_table = ObjectTableSingleton::get_singleton();
        let mut linker = Linker::new(&constant_pool, &object_table);

    let (class_ref, method_index) = linker.link_classes(vec![parent_class, class], "Main", "Main");

    let method_table = NativeMethodTable::get_table();

    let mut vm = Machine::new(&object_table, &method_table, &constant_pool);

    vm.run_bootstrap(class_ref, method_index).unwrap();
}
