
use definitions::{bytecode::{Bytecode, MethodIndex}, class::{ClassHeader, Method, NativeMethodIndex, PoolEntry, PoolIndex, TypeInfo}, object::{Object, Reference}, stack::{Stack, StackUtils}, ArgType, CocoaResult};

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

pub trait ConstantPool {
    fn add_constant(&self, entry: PoolEntry) -> PoolIndex;
    fn set_constant(&self, index: PoolIndex, entry: PoolEntry);
    fn get_constant(&self, index: PoolIndex) -> PoolEntry;
}

pub struct Machine<'a> {
    stack: Stack,
    object_table: &'a dyn ObjectTable,
    method_table: &'a dyn MethodTable,
    constant_pool: &'a dyn ConstantPool,
}

impl<'a> Machine<'a> {
    pub fn new(object_table: &'a dyn ObjectTable, method_table: &'a dyn MethodTable, constant_pool: &'a dyn ConstantPool) -> Machine<'a> {
        Machine {
            stack: Stack::new(),
            object_table,
            method_table,
            constant_pool,
        }
    }
}

impl Machine<'_> {

    fn increment_pc(&mut self) {
        let pc = self.stack.get_current_pc();
        self.stack.set_current_pc(pc + 1);
    }

    fn get_instruction(&self) -> Bytecode {
        let class_ref = self.stack.get_class_index();
        let pc = self.stack.get_current_pc();
        let method_index = self.stack.get_current_method_index();


        let class = self.object_table.get_class(class_ref);

        let method_info = class.get_method(method_index);
        let method_location = method_info.location;

        let method = self.constant_pool.get_constant(method_location);
        let bytecode = match &method {
            PoolEntry::Method(Method::Bytecode(bytecode)) => bytecode,
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
            if self.stack.is_empty() {
                break;
            } 
            let instruction = self.get_instruction();
            self.execute_bytecode(instruction)?;
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
            // Arithmetic
            B::Add => {
                self.stack.add()?;
            },
            B::Subtract => {
                self.stack.subtract()?;
            },
            B::Multiply => {
                self.stack.multiply()?;
            },
            B::Divide => {
                self.stack.divide()?;
            },
            B::Modulo => {
                self.stack.modulo()?;
            },
            B::Negate => {
                self.stack.negate()?;
            },
            // Bitwise
            B::And => {
                self.stack.and()?;
            },
            B::Or => {
                self.stack.or()?;
            },
            B::Xor => {
                self.stack.xor()?;
            },
            B::Not => {
                self.stack.not()?;
            },
            B::ShiftLeft => {
                self.stack.shift_left()?;
            },
            B::ShiftRight => {
                self.stack.shift_right()?;
            },
            // Comparison
            B::Equal => {
                self.stack.equal()?;
            },
            B::Greater => {
                self.stack.greater()?;
            },
            B::Less => {
                self.stack.less()?;
            },
            // Conversion
            B::Convert(ty) => {
                self.stack.convert(ty)?;
            },
            B::BinaryConvert(ty) => {
                self.stack.binary_convert(ty)?;
            },
            // Control Flow
            B::Goto(offset) => {
                let pc = self.stack.get_current_pc();
                self.stack.set_current_pc(((pc as isize) + offset) as usize);
                return Ok(());
            },
            B::If(offset) => {
                let value = StackUtils::<i8>::pop(&mut self.stack);
                if value == 0 {
                    let pc = self.stack.get_current_pc();
                    self.stack.set_current_pc(((pc as isize) + offset) as usize);
                    return Ok(());
                }
            },
            B::IfNot(offset) => {
                let value = StackUtils::<i8>::pop(&mut self.stack);
                if value != 0 {
                    let pc = self.stack.get_current_pc();
                    self.stack.set_current_pc(((pc as isize) + offset) as usize);
                    return Ok(());
                }
            },
            B::IfGreater(offset) => {
                let value = StackUtils::<i8>::pop(&mut self.stack);
                if value > 0 {
                    let pc = self.stack.get_current_pc();
                    self.stack.set_current_pc(((pc as isize) + offset) as usize);
                    return Ok(());
                }
            },
            B::IfLess(offset) => {
                let value = StackUtils::<i8>::pop(&mut self.stack);
                if value < 0 {
                    let pc = self.stack.get_current_pc();
                    self.stack.set_current_pc(((pc as isize) + offset) as usize);
                    return Ok(());
                }
            },
            B::IfGreaterEqual(offset) => {
                let value = StackUtils::<i8>::pop(&mut self.stack);
                if value >= 0 {
                    let pc = self.stack.get_current_pc();
                    self.stack.set_current_pc(((pc as isize) + offset) as usize);
                    return Ok(());
                }
            },
            B::IfLessEqual(offset) => {
                let value = StackUtils::<i8>::pop(&mut self.stack);
                if value <= 0 {
                    let pc = self.stack.get_current_pc();
                    self.stack.set_current_pc(((pc as isize) + offset) as usize);
                    return Ok(());
                }
            },
            B::IfNull(offset) => {
                let value = StackUtils::<Reference>::pop(&mut self.stack);
                StackUtils::<Reference>::push(&mut self.stack, value);
                if value == 0 {
                    let pc = self.stack.get_current_pc();
                    self.stack.set_current_pc(((pc as isize) + offset) as usize);
                    return Ok(());
                }
            },
            B::IfNotNull(offset) => {
                let value = StackUtils::<Reference>::pop(&mut self.stack);
                StackUtils::<Reference>::push(&mut self.stack, value);
                if value != 0 {
                    let pc = self.stack.get_current_pc();
                    self.stack.set_current_pc(((pc as isize) + offset) as usize);
                    return Ok(());
                }
            },
            B::InvokeStatic(method_index) => {
                let class_ref = self.stack.get_class_index();
                let class = self.object_table.get_class(class_ref);
                let method_info = class.get_method(method_index);
                let method = self.constant_pool.get_constant(method_info.location);
                match method {
                    PoolEntry::Method(Method::Native(native_method_index)) => {
                        self.invoke_rust_native_method(class_ref, native_method_index, method_info.type_info)?;
                    },
                    PoolEntry::Method(Method::Bytecode(_)) => {
                        self.increment_pc();
                        self.invoke_bytecode_method(class_ref, method_info.location)?
                    }
                    _ => panic!("Entry is not a method"),
                }
                return Ok(())
            },
            B::InvokeVirtual(method_index) => {
                self.invoke_virtual(method_index)?;
                return Ok(());
            },
            B::Return => {
                self.stack.return_value();
                return Ok(());
            },
            // Object Related
            B::New(pool_index) => {
                let class_ref = self.stack.get_class_index();
                let class = self.object_table.get_class(class_ref);
                let class_info = class.get_constant_pool_entry(pool_index);
                let class_ref = match class_info {
                    PoolEntry::ClassInfo(info) => info.class_ref.unwrap(),
                    _ => panic!("Expected class info"),
                };
                let object_ref = self.object_table.create_object(class_ref);
                self.stack.push(object_ref);
            },
            _ => todo!(),

        }
        self.increment_pc();
        Ok(())
    }

    fn invoke_virtual(&mut self, method_index: MethodIndex) -> CocoaResult<()> {
        let object_ref = StackUtils::<Reference>::pop(&mut self.stack);
        StackUtils::<Reference>::push(&mut self.stack, object_ref);
        let object = self.object_table.get_object(object_ref);
        let class = self.object_table.get_class(object.get_class());

        let method_info = class.get_method(method_index);

        let method = self.constant_pool.get_constant(method_info.location);
        match method {
            PoolEntry::Method(Method::Native(native_method_index)) => {
                self.increment_pc();
                self.invoke_rust_native_method(object.get_class(), native_method_index, method_info.type_info)?;
            }
            PoolEntry::Method(Method::Bytecode(_)) => {
                self.increment_pc();
                self.invoke_bytecode_method(object.get_class(), method_info.location)?;
                return Ok(());
            },
            _ => panic!("Entry is not a method"),
        }
        Ok(())
    }

    fn invoke_bytecode_method(&mut self, class_ref: Reference, method_index: PoolIndex) -> CocoaResult<()> {
        let class = self.object_table.get_class(class_ref);
        let method_info = class.get_method(method_index);

        let method_type_info = self.constant_pool.get_constant(method_info.type_info);
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

    fn invoke_rust_native_method(&mut self, class_ref: Reference, native_method_index: NativeMethodIndex, type_info_index: PoolIndex) -> CocoaResult<()> {
        let method = self.method_table.get_method(native_method_index);

        let type_info = self.constant_pool.get_constant(type_info_index);
        let type_info = match type_info {
            PoolEntry::TypeInfo(info) => info,
            _ => panic!("Expected type info"),
        };

        let (args, _) = match type_info {
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

        let native_method = self.method_table.get_method(native_method_index);
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

    fn print_object(args: &[ArgType]) -> CocoaResult<ArgType> {
        match &args[0] {
            ArgType::Reference(value) => println!("{:?}", value),
            _ => panic!("Expected reference"),
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
            self.objects.borrow().len() - 1 as Reference
        }
    }

    impl ObjectTable for TestObjectTable {
        fn create_object(&self, class_ref: Reference) -> Reference {

            let class = self.get_class(class_ref);

            let parent_info_index = class.get_parent_info();
            let parent_info = class.get_constant_pool_entry(parent_info_index);
            let parent_reference = match parent_info {
                PoolEntry::ClassInfo(ClassInfo {class_ref: Some(class_ref), ..}) => {
                    self.create_object(*class_ref)
                }
                PoolEntry::ClassInfo(ClassInfo {class_ref: None, ..}) => {
                    0
                }
                _ => panic!("Entry was not a class info"),
            };

            // TODO: make this not include static members
            let field_count = class.fields_count();

            let object = Object::new(parent_reference, class_ref, field_count);

            self.objects.borrow_mut().push(object);
            self.objects.borrow().len() - 1 as Reference
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

    #[test]
    fn test_object_creation_and_method() {
        let mut class = ClassHeader::new(9, 0, 0, 2);

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
        class.set_constant_pool_entry(2, PoolEntry::String("Main"));
        class.set_constant_pool_entry(3, PoolEntry::String("Object"));
        class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::New(0), Bytecode::InvokeVirtual(5), Bytecode::Return].into(),0)));
        class.set_constant_pool_entry(5, PoolEntry::Method(Method::Native(0, 1)));
        class.set_constant_pool_entry(6, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(7, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![TypeInfo::Object(3)], ret: Box::new(TypeInfo::U64) }));

        class.set_method(0, MethodInfo {
            flags: MethodFlags::Static,
            name: 0,
            type_info: 6,
            location: 4,
        });

        class.set_method(1, MethodInfo {
            flags: MethodFlags::Public,
            name: 0,
            type_info: 7,
            location: 5,
        });


        let object_table = TestObjectTable::new();

        let class_ref = object_table.add_class(class);
        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(print_object));

        let mut vm = Machine::new(&object_table, &method_table);

        vm.run_bootstrap(class_ref, 0).unwrap();
    }
}
