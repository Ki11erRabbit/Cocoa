
use definitions::{bytecode::{Bytecode, MethodIndex, Type}, class::{ClassHeader, FieldFlags, Method, NativeMethodIndex, PoolEntry, PoolIndex, TypeInfo}, object::{Array, Object, Reference, StringObject}, stack::{Stack, StackUtils}, ArgType, CocoaResult};

use crate::virtual_machine::NativeMethod;


pub trait ObjectTable {
    fn create_object(&self, class_ref: Reference) -> Reference;
    fn add_class(&self, class: ClassHeader) -> Reference;
    fn get_object(&self, object_ref: Reference) -> Object;
    fn get_class(&self, class_ref: Reference) -> ClassHeader;
    fn create_array(&self, ty: Type, length: usize) -> Reference;
    fn get_array(&self, reference: Reference) -> Array;
    fn create_string(&self, string: String) -> Reference;
    fn get_string(&self, reference: Reference) -> StringObject;
    fn is_object(&self, reference: Reference) -> bool;
    fn is_array(&self, reference: Reference) -> bool;
    fn is_class(&self, reference: Reference) -> bool;
    fn is_string(&self, reference: Reference) -> bool;
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
                self.increment_pc();
                match method {
                    PoolEntry::Method(Method::Native(native_method_index)) => {
                        self.invoke_rust_native_method(class_ref, native_method_index, method_info.type_info)?;
                    },
                    PoolEntry::Method(Method::Bytecode(_)) => {
                        self.invoke_bytecode_method(class_ref, method_index)?
                    }
                    _ => panic!("Entry is not a method"),
                }
                return Ok(())
            },
            B::InvokeVirtual(method_index) => {
                let object_ref = StackUtils::<Reference>::pop(&mut self.stack);
                StackUtils::<Reference>::push(&mut self.stack, object_ref);
                self.invoke_virtual(object_ref, method_index)?;
                return Ok(());
            },
            B::InvokeInterface(class_pool_entry, method_index) => {
                let class_ref = self.stack.get_class_index();
                let class = self.object_table.get_class(class_ref);
                let redirect = class.get_constant_pool_entry(class_pool_entry);
                let redirect = match redirect {
                    PoolEntry::Redirect(pool_index) => pool_index,
                    x => panic!("Expected redirect {:?}", x),
                };
                let interface_class = self.constant_pool.get_constant(*redirect);
                let interface_class = match interface_class {
                    PoolEntry::ClassInfo(info) => info,
                    _ => panic!("Expected class info"),
                };

                let interface_name = interface_class.name;

                for interface_info in class.interfaces() {
                    let interface = self.constant_pool.get_constant(interface_info.info);
                    let interface = match interface {
                        PoolEntry::ClassInfo(info) => info,
                        _ => panic!("Expected class info"),
                    };
                    if interface.name == interface_name {
                        let object_ref = StackUtils::<Reference>::pop(&mut self.stack);
                        StackUtils::<Reference>::push(&mut self.stack, object_ref);

                        let method_index = interface_info.vtable[method_index];
                        
                        self.invoke_virtual(object_ref, method_index)?;
                        return Ok(());
                    }
                }

                todo!("error out on interface not found");
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
            B::SetField(field_index) => {
                let object_ref = StackUtils::<Reference>::pop(&mut self.stack);
                let object = self.object_table.get_object(object_ref);
                let class = self.object_table.get_class(object.get_class());
                let field_info = class.get_field(field_index);
                let type_info = self.constant_pool.get_constant(field_info.type_info);

                match type_info {
                    PoolEntry::TypeInfo(TypeInfo::U8) => {
                        let value = StackUtils::<u8>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::U16) => {
                        let value = StackUtils::<u16>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::U32) => {
                        let value = StackUtils::<u32>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::U64) => {
                        let value = StackUtils::<u64>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::I8) => {
                        let value = StackUtils::<i8>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::I16) => {
                        let value = StackUtils::<i16>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::I32) => {
                        let value = StackUtils::<i32>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::I64) => {
                        let value = StackUtils::<i64>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::F32) => {
                        let value = StackUtils::<f32>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::F64) => {
                        let value = StackUtils::<f64>::pop(&mut self.stack);
                        object.set_field(field_index, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::Object(_)) => {
                        let object_ref = StackUtils::<Reference>::pop(&mut self.stack);
                        object.set_field(field_index, object_ref);
                    },
                    _ => todo!(),
                }

                StackUtils::<Reference>::push(&mut self.stack, object_ref);

            },
            B::GetField(field_index) => {
                //TODO: program array members
                let object_ref = StackUtils::<Reference>::pop(&mut self.stack);
                let object = self.object_table.get_object(object_ref);
                let class = self.object_table.get_class(object.get_class());
                let field_info = class.get_field(field_index);
                let type_info = self.constant_pool.get_constant(field_info.type_info);

                match type_info {
                    PoolEntry::TypeInfo(TypeInfo::U8) => {
                        let value = object.get_field::<u8>(field_index);
                        StackUtils::<u8>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::U16) => {
                        let value = object.get_field::<u16>(field_index);
                        StackUtils::<u16>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::U32) => {
                        let value = object.get_field::<u32>(field_index);
                        StackUtils::<u32>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::U64) => {
                        let value = object.get_field::<u64>(field_index);
                        StackUtils::<u64>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::I8) => {
                        let value = object.get_field::<i8>(field_index);
                        StackUtils::<i8>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::I16) => {
                        let value = object.get_field::<i16>(field_index);
                        StackUtils::<i16>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::I32) => {
                        let value = object.get_field::<i32>(field_index);
                        StackUtils::<i32>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::I64) => {
                        let value = object.get_field::<i64>(field_index);
                        StackUtils::<i64>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::F32) => {
                        let value = object.get_field::<f32>(field_index);
                        StackUtils::<f32>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::F64) => {
                        let value = object.get_field::<f64>(field_index);
                        StackUtils::<f64>::push(&mut self.stack, value);
                    },
                    PoolEntry::TypeInfo(TypeInfo::Object(_)) => {
                        let value = object.get_field::<Reference>(field_index);
                        StackUtils::<Reference>::push(&mut self.stack, value);
                    },
                    _ => todo!(),
                }

                StackUtils::<Reference>::push(&mut self.stack, object_ref);
            }
            B::StoreStatic(field_index) => {
                let class_ref = self.stack.get_class_index();
                let class = self.object_table.get_class(class_ref);
                let field_info = class.get_field(field_index);
                let type_info = self.constant_pool.get_constant(field_info.type_info);
                let Some(pool_index) = field_info.location else {
                    todo!("Error out on not having a default value");
                };
                if field_info.flags.contains(FieldFlags::Const) {
                    todo!("Error out on trying to set a constant field");
                }
                if !field_info.flags.contains(FieldFlags::Static) {
                    todo!("Error out on trying to set a non-static field");
                }
                

                match type_info {
                    PoolEntry::TypeInfo(TypeInfo::U8) => {
                        let value = StackUtils::<u8>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::U8(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::U16) => {
                        let value = StackUtils::<u16>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::U16(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::U32) => {
                        let value = StackUtils::<u32>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::U32(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::U64) => {
                        let value = StackUtils::<u64>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::U64(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::I8) => {
                        let value = StackUtils::<i8>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::I8(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::I16) => {
                        let value = StackUtils::<i16>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::I16(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::I32) => {
                        let value = StackUtils::<i32>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::I32(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::I64) => {
                        let value = StackUtils::<i64>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::I64(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::F32) => {
                        let value = StackUtils::<f32>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::F32(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::F64) => {
                        let value = StackUtils::<f64>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::F64(value));
                    },
                    PoolEntry::TypeInfo(TypeInfo::Object(_)) => {
                        let value = StackUtils::<Reference>::pop(&mut self.stack);
                        self.constant_pool.set_constant(pool_index, PoolEntry::Reference(value));
                    },
                    _ => todo!(),
                }
            }
            B::LoadStatic(field_index) => {
                let class_ref = self.stack.get_class_index();
                let class = self.object_table.get_class(class_ref);
                let field_info = class.get_field(field_index);
                let type_info = self.constant_pool.get_constant(field_info.type_info);
                let type_info = match type_info {
                    PoolEntry::TypeInfo(info) => info,
                    _ => panic!("Expected type info"),
                };
                let Some(pool_index) = field_info.location else {
                    todo!("Error out on not having a default value");
                };
                if !field_info.flags.contains(FieldFlags::Static) {
                    todo!("Error out on trying to set a non-static field");
                }

                let value = self.constant_pool.get_constant(pool_index);

                match (value, type_info) {
                    (PoolEntry::U8(value), TypeInfo::U8) => StackUtils::<u8>::push(&mut self.stack, value),
                    (PoolEntry::U16(value), TypeInfo::U16) => StackUtils::<u16>::push(&mut self.stack, value),
                    (PoolEntry::U32(value), TypeInfo::U32) => StackUtils::<u32>::push(&mut self.stack, value),
                    (PoolEntry::U64(value), TypeInfo::U64) => StackUtils::<u64>::push(&mut self.stack, value),
                    (PoolEntry::I8(value), TypeInfo::I8) => StackUtils::<i8>::push(&mut self.stack, value),
                    (PoolEntry::I16(value), TypeInfo::I16) => StackUtils::<i16>::push(&mut self.stack, value),
                    (PoolEntry::I32(value), TypeInfo::I32) => StackUtils::<i32>::push(&mut self.stack, value),
                    (PoolEntry::I64(value), TypeInfo::I64) => StackUtils::<i64>::push(&mut self.stack, value),
                    (PoolEntry::F32(value), TypeInfo::F32) => StackUtils::<f32>::push(&mut self.stack, value),
                    (PoolEntry::F64(value), TypeInfo::F64) => StackUtils::<f64>::push(&mut self.stack, value),
                    (PoolEntry::Reference(value), TypeInfo::Object(_)) => StackUtils::<Reference>::push(&mut self.stack, value),
                    _ => todo!(),
                }
            }
            B::InstanceOf(pool_index) => {
                let object_ref = StackUtils::<Reference>::pop(&mut self.stack);
                StackUtils::<Reference>::push(&mut self.stack, object_ref);
                self.instance_of(object_ref, pool_index);
            }
            // Array Related
            B::NewArray(ty) => {
                let length = StackUtils::<u64>::pop(&mut self.stack) as usize;
                let reference = self.object_table.create_array(ty, length);
                StackUtils::<Reference>::push(&mut self.stack, reference);
            }
            B::ArrayGet(ty) => {
                let index = StackUtils::<u64>::pop(&mut self.stack) as usize;
                let reference = StackUtils::<Reference>::pop(&mut self.stack);
                StackUtils::<Reference>::push(&mut self.stack, reference);
                // TODO: check if array
                let array = self.object_table.get_array(reference);
                match ty {
                    Type::U8 => {
                        let value = array.get_elem::<u8>(index);
                        self.stack.push(value);
                    }
                    Type::I8 => {
                        let value = array.get_elem::<i8>(index);
                        self.stack.push(value);
                    }
                    Type::U16 => {
                        let value = array.get_elem::<u16>(index);
                        self.stack.push(value);
                    }
                    Type::I16 => {
                        let value = array.get_elem::<i16>(index);
                        self.stack.push(value);
                    }
                    Type::U32 => {
                        let value = array.get_elem::<u32>(index);
                        self.stack.push(value);
                    }
                    Type::I32 => {
                        let value = array.get_elem::<i32>(index);
                        self.stack.push(value);
                    }
                    Type::U64 => {
                        let value = array.get_elem::<u64>(index);
                        self.stack.push(value);
                    }
                    Type::I64 => {
                        let value = array.get_elem::<i64>(index);
                        self.stack.push(value);
                    }
                    Type::F32 => {
                        let value = array.get_elem::<f32>(index);
                        self.stack.push(value);
                    }
                    Type::F64 => {
                        let value = array.get_elem::<f64>(index);
                        self.stack.push(value);
                    }
                    Type::Reference => {
                        let value = array.get_elem::<Reference>(index);
                        self.stack.push(value);
                    }
                    _ => panic!("invalid size "),
                }
            }
            B::ArraySet(ty) => {
                let index = StackUtils::<u64>::pop(&mut self.stack) as usize;
                let reference = StackUtils::<Reference>::pop(&mut self.stack);
                // TODO: check if array
                let mut array = self.object_table.get_array(reference);
                match ty {
                    Type::U8 => {
                        let value = self.stack.pop();
                        array.set_elem::<u8>(index, value);
                    }
                    Type::I8 => {
                        let value = self.stack.pop();
                        array.set_elem::<i8>(index, value);
                    }
                    Type::U16 => {
                        let value = self.stack.pop();
                        array.set_elem::<u16>(index, value);
                    }
                    Type::I16 => {
                        let value = self.stack.pop();
                        array.set_elem::<i16>(index, value);
                    }
                    Type::U32 => {
                        let value = self.stack.pop();
                        array.set_elem::<u32>(index, value);
                    }
                    Type::I32 => {
                        let value = self.stack.pop();
                        array.set_elem::<i32>(index, value);
                    }
                    Type::U64 => {
                        let value = self.stack.pop();
                        array.set_elem::<u64>(index, value);
                    }
                    Type::I64 => {
                        let value = self.stack.pop();
                        array.set_elem::<i64>(index, value);
                    }
                    Type::F32 => {
                        let value = self.stack.pop();
                        array.set_elem::<f32>(index, value);
                    }
                    Type::F64 => {
                        let value = self.stack.pop();
                        array.set_elem::<f64>(index, value);
                    }
                    Type::Reference => {
                        let value = self.stack.pop();
                        array.set_elem::<Reference>(index, value);
                    }
                    _ => panic!("invalid size "),
                }
                StackUtils::<Reference>::push(&mut self.stack, reference);
            }
            // String Related
            B::NewString(string_index) => {
                let class_ref = self.stack.get_class_index();
                let class = self.object_table.get_class(class_ref);
                let pool_index = class.get_string(string_index);
                let string = self.constant_pool.get_constant(pool_index);
                let string = match string {
                    PoolEntry::String(string) => string,
                    _ => panic!("Was not a string"),
                };

                let string_ref = self.object_table.create_string(string);
                self.stack.push(string_ref);
            }
            // Misc
            B::Breakpoint => {
                todo!("breakpoint")
            }
            B::Nop => {}
            
        }
        self.increment_pc();
        Ok(())
    }

    fn instance_of(&mut self, object_ref: Reference, pool_index: PoolIndex) {
        let object = self.object_table.get_object(object_ref);
        let class_ref = object.get_class();

        let stack_class_index = self.stack.get_class_index();
        let stack_class = self.object_table.get_class(stack_class_index);
        let class_info = stack_class.get_constant_pool_entry(pool_index);
        let class_info = match class_info {
            PoolEntry::ClassInfo(info) => info,
            x => panic!("Expected redirect {:?}", x),
        };

        if class_ref == class_info.class_ref.unwrap() {
            StackUtils::<i8>::push(&mut self.stack, 0);
        } else if object.get_parent() != 0 {
            self.instance_of(object.get_parent(), pool_index);
        } else {
            StackUtils::<i8>::push(&mut self.stack, 1);
        }
    }

    fn invoke_virtual(&mut self, object_ref: Reference, method_index: MethodIndex) -> CocoaResult<()> {
        if object_ref == 0 {
            panic!("Attempted to invoke method on null object");
        }
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
                self.invoke_bytecode_method(object.get_class(), method_index)?;
                return Ok(());
            },
            PoolEntry::Method(Method::Foreign(method_index)) => {
                let parent_ref = object.get_parent();
                self.invoke_virtual(parent_ref, method_index)?;
            }
            _ => panic!("Entry is not a method"),
        }
        Ok(())
    }

    fn invoke_bytecode_method(&mut self, class_ref: Reference, method_index: MethodIndex) -> CocoaResult<()> {
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
    use sequential_test::sequential;
    use std::cell::RefCell;
    use crate::virtual_machine::NativeMethod;
    use definitions::{bytecode::Bytecode, class::{ClassHeader, ClassInfo, InterfaceInfo, Method, MethodFlags, MethodInfo, PoolEntry, PoolIndex}, object::{Object, Reference}, ArgType};
    use crate::ConstantPoolSingleton;
    use crate::virtual_machine::Linker;


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
                objects: RefCell::new(vec![Object::new(0,0,0)]),
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
                x => panic!("Entry was not a class info {:?}", x),
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
            if object_ref == 0 {
                panic!("Attempted to get object 0");
            }
                       
            self.objects.borrow()[object_ref as usize].clone()
        }

        fn get_class(&self, class_ref: Reference) -> ClassHeader {
            self.classes.borrow()[class_ref as usize].clone()
        }

        fn create_array(&self, ty: Type, length: usize) -> Reference {
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
    #[sequential]
    fn test_hello_world() {
        let mut class = ClassHeader::new(9, 0, 0, 2, 0);

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
        class.set_constant_pool_entry(8, PoolEntry::String("helloWorld".to_owned()));

        class.set_method(0, MethodInfo {
            flags: MethodFlags::Static,
            name: 2,
            type_info: 6,
            location: 4,
        });

        class.set_method(1, MethodInfo {
            flags: MethodFlags::Static,
            name: 8,
            type_info: 7,
            location: 5,
        });
        
        let constant_pool = ConstantPoolSingleton::new();
        let object_table = TestObjectTable::new();
        let mut linker = Linker::new(&constant_pool, &object_table);

        let (class_ref, method_index) = linker.link_classes(vec![class], "Main", "Main");

        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(hello_world));
        

        let mut vm = Machine::new(&object_table, &method_table, &constant_pool);

        vm.run_bootstrap(class_ref, method_index).unwrap();
    }

    #[test]
    #[sequential]
    fn test_print_i32() {
        let mut class = ClassHeader::new(10, 0, 0, 2, 0);

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
        class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::LoadConstant(8), Bytecode::InvokeStatic(1), Bytecode::Return].into())));
        class.set_constant_pool_entry(5, PoolEntry::Method(Method::Native(0)));
        class.set_constant_pool_entry(6, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(7, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![TypeInfo::I32], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(8, PoolEntry::I32(42));
        class.set_constant_pool_entry(9, PoolEntry::String("print".to_owned()));

        class.set_method(0, MethodInfo {
            flags: MethodFlags::Static,
            name: 2,
            type_info: 6,
            location: 4,
        });

        class.set_method(1, MethodInfo {
            flags: MethodFlags::Static,
            name: 9,
            type_info: 7,
            location: 5,
        });

        let constant_pool = ConstantPoolSingleton::new();
        let object_table = TestObjectTable::new();
        let mut linker = Linker::new(&constant_pool, &object_table);

        let (class_ref, method_index) = linker.link_classes(vec![class], "Main", "Main");

        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(print_i32));

        let mut vm = Machine::new(&object_table, &method_table, &constant_pool);

        vm.run_bootstrap(class_ref, method_index).unwrap();
    }

    #[test]
    #[sequential]
    fn test_object_creation_and_method() {
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

        let constant_pool = ConstantPoolSingleton::new();
        let object_table = TestObjectTable::new();
        let mut linker = Linker::new(&constant_pool, &object_table);

        let (class_ref, method_index) = linker.link_classes(vec![class], "Main", "Main");

        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(print_object));

        let mut vm = Machine::new(&object_table, &method_table, &constant_pool);

        vm.run_bootstrap(class_ref, method_index).unwrap();
    }

    #[test]
    #[sequential]
    fn test_object_inheritance() {
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
        let object_table = TestObjectTable::new();
        let mut linker = Linker::new(&constant_pool, &object_table);

        let (class_ref, method_index) = linker.link_classes(vec![parent_class, class], "Main", "Main");

        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(print_object));

        let mut vm = Machine::new(&object_table, &method_table, &constant_pool);

        vm.run_bootstrap(class_ref, method_index).unwrap();
    }

    #[test]
    #[sequential]
    fn test_interface() {
        let mut class = ClassHeader::new(11, 1, 0, 2, 0);

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
        class.set_constant_pool_entry(3, PoolEntry::String("Object".to_owned()));
        class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::New(0), Bytecode::InvokeInterface(9, 0), Bytecode::Return].into())));
        class.set_constant_pool_entry(5, PoolEntry::Method(Method::Native(0)));
        class.set_constant_pool_entry(6, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(7, PoolEntry::TypeInfo(TypeInfo::Method { args: vec![TypeInfo::Object(3)], ret: Box::new(TypeInfo::U64) }));
        class.set_constant_pool_entry(8, PoolEntry::String("printRef".to_owned()));
        class.set_constant_pool_entry(9, PoolEntry::ClassInfo(ClassInfo {
            name: 10,
            class_ref: None,
        }));
        class.set_constant_pool_entry(10, PoolEntry::String("PrintRef".to_owned()));

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

        class.set_interface(0, InterfaceInfo {
            info: 9,
            vtable: vec![1],
        });

        let constant_pool = ConstantPoolSingleton::new();
        let object_table = TestObjectTable::new();
        let mut linker = Linker::new(&constant_pool, &object_table);

        let (class_ref, method_index) = linker.link_classes(vec![class], "Main", "Main");

        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(print_object));

        let mut vm = Machine::new(&object_table, &method_table, &constant_pool);

        vm.run_bootstrap(class_ref, method_index).unwrap();
    }

    #[test]
    #[sequential]
    fn test_object_instance_of() {
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
        class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::New(0), Bytecode::InstanceOf(0), Bytecode::IfNot(10), Bytecode::Return].into())));
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
        let object_table = TestObjectTable::new();
        let mut linker = Linker::new(&constant_pool, &object_table);

        let (class_ref, method_index) = linker.link_classes(vec![parent_class, class], "Main", "Main");

        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(print_object));

        let mut vm = Machine::new(&object_table, &method_table, &constant_pool);

        vm.run_bootstrap(class_ref, method_index).unwrap();
    }

    #[test]
    #[sequential]
    fn test_object_instance_of_inheritance() {
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
        class.set_constant_pool_entry(4, PoolEntry::Method(Method::Bytecode(vec![Bytecode::New(0), Bytecode::InstanceOf(1), Bytecode::IfNot(10), Bytecode::Return].into())));
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
        let object_table = TestObjectTable::new();
        let mut linker = Linker::new(&constant_pool, &object_table);

        let (class_ref, method_index) = linker.link_classes(vec![parent_class, class], "Main", "Main");

        let mut method_table = TestMethodTable::new();

        method_table.add_method(NativeMethod::Rust(print_object));

        let mut vm = Machine::new(&object_table, &method_table, &constant_pool);

        vm.run_bootstrap(class_ref, method_index).unwrap();
    }
}
