use definitions::{bytecode::Bytecode, class::{ClassHeader, Method, NativeMethodIndex, PoolEntry, PoolIndex, TypeInfo}, object::{Object, Reference}, stack::{Stack, StackUtils}, ArgType, CocoaResult};

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
            B::LoadConstant(pool_index) => {
                let class_ref = self.stack.get_class_index();
                let class = self.object_table.get_class(class_ref);
                let constant = class.get_constant_pool_entry(pool_index);
                match constant {
                    PoolEntry::U8(value) => self.stack.push(*value),
                    PoolEntry::U16(value) => self.stack.push(*value),
                    PoolEntry::U32(value) => self.stack.push(*value),
                    PoolEntry::U64(value) => self.stack.push(*value),
                    PoolEntry::I8(value) => self.stack.push(*value),
                    PoolEntry::I16(value) => self.stack.push(*value),
                    PoolEntry::I32(value) => self.stack.push(*value),
                    PoolEntry::I64(value) => self.stack.push(*value),
                    PoolEntry::F32(value) => self.stack.push(*value),
                    PoolEntry::F64(value) => self.stack.push(*value),
                    PoolEntry::String(_) => todo!("Create string object"),
                    _ => todo!(),
                }
            },
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
                    PoolEntry::Method(Method::Native(_,_)) => {
                        self.invoke_rust_native_method(class_ref, pool_index)?;
                    }
                    PoolEntry::Method(Method::Bytecode(_, method_index)) => {

                        let pc = self.stack.get_current_pc();
                        self.stack.set_current_pc(pc + 1);
                        self.invoke_bytecode_method(self.stack.get_class_index(), *method_index)?;
                        return Ok(());
                    },
                    _ => panic!("Entry is not a method"),
                }

            },
            B::Return => {
                self.stack.return_value();
                return Ok(());
            },
            _ => todo!(),

        }

        let pc = self.stack.get_current_pc();
        self.stack.set_current_pc(pc + 1);
        Ok(())
    }

    fn invoke_bytecode_method(&mut self, class_ref: Reference, method_index: PoolIndex) -> CocoaResult<()> {
        let class = self.object_table.get_class(class_ref);
        let method_info = class.get_method(method_index);

        let method_type_info = class.get_constant_pool_entry(method_info.type_info);
        let method_type_info = match method_type_info {
            PoolEntry::TypeInfo(info) => info,
            _ => panic!("Expected type info"),
        };

        let (args, _) = match method_type_info {
            TypeInfo::Method { args, ret } => (args, ret),
            _ => panic!("Expected method type info"),
        };

        self.stack.push_frame(class_ref, method_index);
        let mut arg_index = 0;
        for arg in args {
            match arg {
                TypeInfo::U8 => StackUtils::<u8>::set_argument(&mut self.stack, arg_index),
                TypeInfo::U16 => StackUtils::<u16>::set_argument(&mut self.stack, arg_index),
                TypeInfo::U32 => StackUtils::<u32>::set_argument(&mut self.stack, arg_index),
                TypeInfo::U64 => StackUtils::<u64>::set_argument(&mut self.stack, arg_index),
                TypeInfo::I8 => StackUtils::<i8>::set_argument(&mut self.stack, arg_index),
                TypeInfo::I16 => StackUtils::<i16>::set_argument(&mut self.stack, arg_index),
                TypeInfo::I32 => StackUtils::<i32>::set_argument(&mut self.stack, arg_index),
                TypeInfo::I64 => StackUtils::<i64>::set_argument(&mut self.stack, arg_index),
                TypeInfo::F32 => StackUtils::<f32>::set_argument(&mut self.stack, arg_index),
                TypeInfo::F64 => StackUtils::<f64>::set_argument(&mut self.stack, arg_index),
                TypeInfo::Object(_) => StackUtils::<Reference>::set_argument(&mut self.stack, arg_index),
                _ => todo!(),
            }
            arg_index += 1;
        }
        
        Ok(())
    }

    fn invoke_rust_native_method(&mut self, class_ref: Reference, pool_index: PoolIndex) -> CocoaResult<()> {
        let class = self.object_table.get_class(class_ref);
        let method = class.get_constant_pool_entry(pool_index);
        match method {
            PoolEntry::Method(Method::Native(native_method_index, method_index)) => {
                let method_info = class.get_method(*method_index);

                let method_type_info = class.get_constant_pool_entry(method_info.type_info);
                let method_type_info = match method_type_info {
                    PoolEntry::TypeInfo(info) => info,
                    _ => panic!("Expected type info"),
                };

                let (args, _) = match method_type_info {
                    TypeInfo::Method { args, ret } => (args, ret),
                    _ => panic!("Expected method type info"),
                };

                let mut method_args = Vec::new();

                for arg in args {
                    match arg {
                        TypeInfo::U8 => {
                            method_args.push(ArgType::U8(StackUtils::<u8>::pop(&mut self.stack)));
                        }
                        TypeInfo::U16 => {
                            method_args.push(ArgType::U16(StackUtils::<u16>::pop(&mut self.stack)));
                        }
                        TypeInfo::U32 => {
                            method_args.push(ArgType::U32(StackUtils::<u32>::pop(&mut self.stack)));
                        }
                        TypeInfo::U64 => {
                            method_args.push(ArgType::U64(StackUtils::<u64>::pop(&mut self.stack)));
                        }
                        TypeInfo::I8 => {
                            method_args.push(ArgType::I8(StackUtils::<i8>::pop(&mut self.stack)));
                        }
                        TypeInfo::I16 => {
                            method_args.push(ArgType::I16(StackUtils::<i16>::pop(&mut self.stack)));
                        }
                        TypeInfo::I32 => {
                            method_args.push(ArgType::I32(StackUtils::<i32>::pop(&mut self.stack)));
                        }
                        TypeInfo::I64 => {
                            method_args.push(ArgType::I64(StackUtils::<i64>::pop(&mut self.stack)));
                        }
                        TypeInfo::F32 => {
                            method_args.push(ArgType::F32(StackUtils::<f32>::pop(&mut self.stack)));
                        }
                        TypeInfo::F64 => {
                            method_args.push(ArgType::F64(StackUtils::<f64>::pop(&mut self.stack)));
                        }
                        TypeInfo::Object(_) => {
                            method_args.push(ArgType::Reference(StackUtils::<Reference>::pop(&mut self.stack)));
                        }
                        _ => todo!(),
                    }
                }
                
                let native_method = self.method_table.get_method(*native_method_index);
                match native_method {
                    NativeMethod::Rust(method) => {
                        let value = method(&method_args)?;
                        match value {
                            ArgType::U8(value) => self.stack.push(value),
                            ArgType::U16(value) => self.stack.push(value),
                            ArgType::U32(value) => self.stack.push(value),
                            ArgType::U64(value) => self.stack.push(value),
                            ArgType::I8(value) => self.stack.push(value),
                            ArgType::I16(value) => self.stack.push(value),
                            ArgType::I32(value) => self.stack.push(value),
                            ArgType::I64(value) => self.stack.push(value),
                            ArgType::F32(value) => self.stack.push(value),
                            ArgType::F64(value) => self.stack.push(value),
                            ArgType::Reference(value) => self.stack.push(value),
                            ArgType::Unit => (),
                        }
                            
                    },
                }
            }
            _ => panic!("Entry is not a method"),
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use crate::virtual_machine::NativeMethod;
    use definitions::{bytecode::Bytecode, class::{ClassHeader, ClassInfo, Method, MethodFlags, MethodInfo, PoolEntry, PoolIndex}, object::{Object, Reference}, ArgType};


    fn hello_world(_: &[ArgType]) -> CocoaResult<ArgType> {
        println!("Hello, world!");
        Ok(ArgType::U64(0))
    }

    fn print_i32(args: &[ArgType]) -> CocoaResult<ArgType> {
        match &args[0] {
            ArgType::I32(value) => println!("{}", value),
            _ => panic!("Expected i32"),
        }
        Ok(ArgType::U64(0))
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
        class.set_constant_pool_entry(2, PoolEntry::String("Main"));
        class.set_constant_pool_entry(3, PoolEntry::String("Object"));
        class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::InvokeStatic(5), Bytecode::Return].into(),0)));
        class.set_constant_pool_entry(5, PoolEntry::Method(Method::Native(0, 1)));
        class.set_constant_pool_entry(6, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(7, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));

        class.set_method(0, MethodInfo {
            flags: MethodFlags::Static,
            name: 0,
            type_info: 6,
            location: 4,
        });

        class.set_method(1, MethodInfo {
            flags: MethodFlags::Static,
            name: 0,
            type_info: 7,
            location: 5,
        });


        let object_table = TestObjectTable::new();

        let class_ref = object_table.add_class(class);
        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(hello_world));

        let mut vm = Machine::new(&object_table, &method_table);

        vm.run_bootstrap(class_ref, 0).unwrap();
    }

    #[test]
    fn test_print_i32() {
        let mut class = ClassHeader::new(9, 0, 0, 2);

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
        class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::LoadConstant(8), Bytecode::InvokeStatic(5), Bytecode::Return].into(),0)));
        class.set_constant_pool_entry(5, PoolEntry::Method(Method::Native(0, 1)));
        class.set_constant_pool_entry(6, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(7, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![TypeInfo::I32], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(8, PoolEntry::I32(42));

        class.set_method(0, MethodInfo {
            flags: MethodFlags::Static,
            name: 0,
            type_info: 6,
            location: 4,
        });

        class.set_method(1, MethodInfo {
            flags: MethodFlags::Static,
            name: 0,
            type_info: 7,
            location: 5,
        });


        let object_table = TestObjectTable::new();

        let class_ref = object_table.add_class(class);
        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(print_i32));

        let mut vm = Machine::new(&object_table, &method_table);

        vm.run_bootstrap(class_ref, 0).unwrap();
    }

}
