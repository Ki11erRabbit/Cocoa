use definitions::{bytecode::Bytecode, class::{ClassHeader, Method, PoolEntry, PoolIndex}, object::{Object, Reference}, stack::{Stack, StackUtils}, CocoaResult};

use crate::virtual_machine::NativeMethod;


pub trait ObjectTable {
    fn create_object(&self, class_ref: Reference) -> Reference;
    fn add_class(&self, class: ClassHeader) -> Reference;
    fn get_object(&self, object_ref: Reference) -> Object;
    fn get_class(&self, class_ref: Reference) -> ClassHeader;
}

pub trait MethodTable {
    fn get_method(&self, method_index: PoolIndex) -> NativeMethod;
}

pub struct Machine<'a> {
    stack: Stack,
    object_table: &'a dyn ObjectTable,
    method_table: &'a dyn MethodTable,
    
}

impl<'a> Machine<'a> {
    pub fn new(object_table: &'a dyn ObjectTable, method_table: &'a dyn MethodTable) -> Machine<'a> {
        Machine {
            stack: Stack::new(),
            object_table,
            method_table,
        }
    }
}

impl Machine<'_> {

    fn get_instruction(&self) -> Bytecode {
        let class_ref = self.stack.get_class_index();
        let pc = self.stack.get_current_pc();
        let method_index = self.stack.get_current_method_index();


        let class = self.object_table.get_class(class_ref);

        let method_info = class.get_method(method_index);
        let method_location = method_info.location;

        let method = class.get_constant_pool_entry(method_location);
        let bytecode = match &method {
            PoolEntry::Method(Method::Bytecode(bytecode, _)) => bytecode,
            x => panic!("Entry is not a method {:?}", x),
        };
        
        bytecode[pc]
    }

    pub fn run_bootstrap(&mut self, main_class_ref: Reference, main_method_index: PoolIndex) -> CocoaResult<()> {
        self.stack.push_frame(main_class_ref, main_method_index);
        
        self.run()
    }

    pub fn run(&mut self) -> CocoaResult<()> {
        loop {
            let instruction = self.get_instruction();
            self.execute_bytecode(instruction)?;
            if self.stack.is_empty() {
                break;
            }
        }
        Ok(())
    }

    fn execute_bytecode(&mut self, code: Bytecode) -> CocoaResult<()> {
        use Bytecode as B;
        match code {
            // Stack Manipulation
            B::Pop => self.stack.generic_pop(),
            B::PushNull => self.stack.push(0 as Reference),
            B::LoadConstant(pool_index) => todo!(),
            B::Dup => self.stack.dup(),
            B::Swap => self.stack.swap(),
            // Local Variables
            B::StoreLocal(index) => self.stack.set_local(index),
            B::LoadLocal(index) => self.stack.get_local(index),
            // Control Flow
            B::InvokeStatic(pool_index) => {
                let class_ref = self.stack.get_class_index();

                let class = self.object_table.get_class(class_ref);

                let method = class.get_constant_pool_entry(pool_index);
                // TODO: Check if actually static
                match method {
                    PoolEntry::Method(Method::Native(native_method_index)) => {
                        let native_method = self.method_table.get_method(*native_method_index);
                        match native_method {
                            NativeMethod::Rust(method) => {
                                method(&[])?;
                            },
                        }
                    }
                    PoolEntry::Method(Method::Bytecode(_, method_index)) => {

                        let pc = self.stack.get_current_pc();
                        self.stack.set_current_pc(pc + 1);
                        self.stack.push_frame(self.stack.get_class_index(), *method_index);
                        return Ok(());
                    },
                    _ => panic!("Entry is not a method"),
                }

            },
            B::Return => {
                self.stack.pop_frame();
                return Ok(());
            },
            _ => todo!(),

        }

        let pc = self.stack.get_current_pc();
        self.stack.set_current_pc(pc + 1);
        Ok(())
    }
    

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use crate::virtual_machine::NativeMethod;
    use definitions::{bytecode::Bytecode, class::{ClassHeader, ClassInfo, MethodFlags, MethodInfo, PoolEntry, PoolIndex, Method}, object::{Object, Reference}, ReturnType};


    fn hello_world(_: &[Object]) -> CocoaResult<ReturnType> {
        println!("Hello, world!");
        Ok(ReturnType::U64(0))
    }

    struct TestObjectTable {
        objects: RefCell<Vec<Object>>,
        classes: RefCell<Vec<ClassHeader>>,
    }

    impl TestObjectTable {
        fn new() -> Self {
            TestObjectTable {
                objects: RefCell::new(Vec::new()),
                classes: RefCell::new(Vec::new()),
            }
        }

        fn add_object(&mut self, object: Object) -> Reference {
            self.objects.borrow_mut().push(object);
            self.objects.borrow().len() as Reference
        }
    }

    impl ObjectTable for TestObjectTable {
        fn create_object(&self, class_ref: Reference) -> Reference {
            todo!()
        }

        fn add_class(&self, class: ClassHeader) -> Reference {
            self.classes.borrow_mut().push(class);
            self.classes.borrow().len() - 1 as Reference
        }

        fn get_object(&self, object_ref: Reference) -> Object {
            self.objects.borrow()[object_ref as usize].clone()
        }

        fn get_class(&self, class_ref: Reference) -> ClassHeader {
            self.classes.borrow()[class_ref as usize].clone()
        }
    }

    struct TestMethodTable {
        methods: Vec<NativeMethod>,
    }
    impl TestMethodTable {
        fn new() -> Self {
            TestMethodTable {
                methods: Vec::new(),
            }
        }

        fn add_method(&mut self, method: NativeMethod) {
            self.methods.push(method);
        }
    }

    impl MethodTable for TestMethodTable {
        fn get_method(&self, method_index: PoolIndex) -> NativeMethod {
            self.methods[method_index as usize].clone()
        }
    }

    #[test]
    fn test_hello_world() {
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
        class.set_constant_pool_entry(5, PoolEntry::Method(Method::Native(0)));

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


        let object_table = TestObjectTable::new();

        let class_ref = object_table.add_class(class);
        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(hello_world));

        let mut vm = Machine::new(&object_table, &method_table);

        vm.run_bootstrap(class_ref, 0).unwrap();
    }
}
