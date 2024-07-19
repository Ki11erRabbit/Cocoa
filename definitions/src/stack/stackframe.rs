
use crate::{bytecode::Type, object::Reference};

pub trait StackFrameUtils<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> T;
    fn store_argument(&mut self, index: u8, value: T);
    fn load_argument(&mut self, index: u8) -> T;
    fn return_value(&mut self) -> T {
        self.pop()
    }
}


pub type LocalVariable = usize;

pub struct StackFrame {
    local_variables: Vec<LocalVariable>,
    local_variable_types: Vec<Type>,
    operand_stack: Vec<u8>,
    operand_stack_types: Vec<Type>,
    class_reference: Reference,
    method_index: usize,
    pc: usize,
}


impl StackFrame {
    pub fn new(class_reference: Reference, method_index: usize) -> Self {
        Self {
            local_variables: vec![0; u8::MAX as usize],
            local_variable_types: vec![Type::U8; u8::MAX as usize],
            operand_stack: Vec::new(),
            operand_stack_types: Vec::new(),
            class_reference,
            method_index,
            pc: 0,
        }
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    pub fn get_class_reference(&self) -> Reference {
        self.class_reference
    }

    pub fn get_method_index(&self) -> usize {
        self.method_index
    }

    pub fn generic_pop(&mut self) {
        let ty = self.operand_stack_types.pop().expect("stack underflow");
        match ty {
            Type::Char(size) => {
                for _ in 0..size {
                    self.operand_stack.pop();
                }
            }
            Type::U8 | Type::I8 => {self.operand_stack.pop();},
            Type::U16 | Type::I16 => {
                self.operand_stack.pop();
                self.operand_stack.pop();
            }
            Type::U32 | Type::I32 | Type::F32 => {
                self.operand_stack.pop();
                self.operand_stack.pop();
                self.operand_stack.pop();
                self.operand_stack.pop();
            }
            Type::U64 | Type::I64 | Type::F64 | Type::Reference => {
                self.operand_stack.pop();
                self.operand_stack.pop();
                self.operand_stack.pop();
                self.operand_stack.pop();
                self.operand_stack.pop();
                self.operand_stack.pop();
                self.operand_stack.pop();
                self.operand_stack.pop();
            }
        }
    }

    pub fn get_references(&self) -> Vec<Reference> {
        let mut references: Vec<Reference> = self.local_variables.iter().zip(self.local_variable_types.iter()).filter_map(|(x, y)| {
            if *y == Type::Reference {
                Some(*x as Reference)
            } else {
                None
            }
        }).collect();

        let mut type_iter = self.operand_stack_types.iter();

        let mut index = 0;
        
        while let Some(ty) = type_iter.next() {
            match ty {
                Type::Reference => {
                    let val1 = self.operand_stack[index];
                    let val2 = self.operand_stack[index + 1];
                    let val3 = self.operand_stack[index + 2];
                    let val4 = self.operand_stack[index + 3];
                    let val5 = self.operand_stack[index + 4];
                    let val6 = self.operand_stack[index + 5];
                    let val7 = self.operand_stack[index + 6];
                    let val8 = self.operand_stack[index + 7];
                    references.push(u64::from_le_bytes([val1, val2, val3, val4, val5, val6, val7, val8]) as Reference);
                    index += 8;
                },
                Type::Char(size) => {
                    index += *size as usize;
                },
                Type::U8 | Type::I8 => {
                    index += 1;
                },
                Type::U16 | Type::I16 => {
                    index += 2;
                },
                Type::U32 | Type::I32 | Type::F32 => {
                    index += 4;
                },
                Type::U64 | Type::I64 | Type::F64 => {
                    index += 8;
                },
            }
        }

        references
    }

    pub fn swap(&mut self) {
        let top_ty = self.operand_stack_types.pop().expect("Stack underflow");
        let (ty, bytes) = match top_ty {
            Type::U8 | Type::I8 => {
                let val = self.operand_stack.pop().expect("Stack underflow");
                (Type::U8, vec![val])
            },
            Type::U16 | Type::I16 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                (Type::U16, vec![val2, val1])
            },
            Type::U32 | Type::I32 | Type::F32 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                (Type::U32, vec![val4, val3, val2, val1])
            },
            Type::U64 | Type::I64 | Type::F64 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                let val5 = self.operand_stack.pop().expect("Stack underflow");
                let val6 = self.operand_stack.pop().expect("Stack underflow");
                let val7 = self.operand_stack.pop().expect("Stack underflow");
                let val8 = self.operand_stack.pop().expect("Stack underflow");
                (Type::U64, vec![val8, val7, val6, val5, val4, val3, val2, val1])
            },
            Type::Char(size) => {
                let mut val = Vec::new();
                for _ in 0..size {
                    val.push(self.operand_stack.pop().expect("Stack underflow"));
                }
                (Type::Char(size), val)
            },
            Type::Reference => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                let val5 = self.operand_stack.pop().expect("Stack underflow");
                let val6 = self.operand_stack.pop().expect("Stack underflow");
                let val7 = self.operand_stack.pop().expect("Stack underflow");
                let val8 = self.operand_stack.pop().expect("Stack underflow");
                (Type::Reference, vec![val8, val7, val6, val5, val4, val3, val2, val1])
            },
        };
        let bottom_ty = self.operand_stack_types.pop().expect("Stack underflow");
        let (ty2, bytes2) = match bottom_ty {
            Type::U8 | Type::I8 => {
                let val = self.operand_stack.pop().expect("Stack underflow");
                (Type::U8, vec![val])
            },
            Type::U16 | Type::I16 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                (Type::U16, vec![val2, val1])
            },
            Type::U32 | Type::I32 | Type::F32 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                (Type::U32, vec![val4, val3, val2, val1])
            },
            Type::U64 | Type::I64 | Type::F64 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                let val5 = self.operand_stack.pop().expect("Stack underflow");
                let val6 = self.operand_stack.pop().expect("Stack underflow");
                let val7 = self.operand_stack.pop().expect("Stack underflow");
                let val8 = self.operand_stack.pop().expect("Stack underflow");
                (Type::U64, vec![val8, val7, val6, val5, val4, val3, val2, val1])
            },
            Type::Char(size) => {
                let mut val = Vec::new();
                for _ in 0..size {
                    val.push(self.operand_stack.pop().expect("Stack underflow"));
                }
                (Type::Char(size), val)
            },
            Type::Reference => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                let val5 = self.operand_stack.pop().expect("Stack underflow");
                let val6 = self.operand_stack.pop().expect("Stack underflow");
                let val7 = self.operand_stack.pop().expect("Stack underflow");
                let val8 = self.operand_stack.pop().expect("Stack underflow");
                (Type::Reference, vec![val8, val7, val6, val5, val4, val3, val2, val1])
            },
        };

        self.operand_stack_types.push(ty);
        self.operand_stack_types.push(ty2);
        self.operand_stack.extend_from_slice(&bytes);
        self.operand_stack.extend_from_slice(&bytes2);
    }

    pub fn dup(&mut self) {
        let top_ty = self.operand_stack_types.pop().expect("Stack underflow");
        let (ty, bytes) = match top_ty {
            Type::U8 | Type::I8 => {
                let val = self.operand_stack.pop().expect("Stack underflow");
                (Type::U8, vec![val])
            },
            Type::U16 | Type::I16 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                (Type::U16, vec![val2, val1])
            },
            Type::U32 | Type::I32 | Type::F32 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                (Type::U32, vec![val4, val3, val2, val1])
            },
            Type::U64 | Type::I64 | Type::F64 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                let val5 = self.operand_stack.pop().expect("Stack underflow");
                let val6 = self.operand_stack.pop().expect("Stack underflow");
                let val7 = self.operand_stack.pop().expect("Stack underflow");
                let val8 = self.operand_stack.pop().expect("Stack underflow");
                (Type::U64, vec![val8, val7, val6, val5, val4, val3, val2, val1])
            },
            Type::Char(size) => {
                let mut val = Vec::new();
                for _ in 0..size {
                    val.push(self.operand_stack.pop().expect("Stack underflow"));
                }
                (Type::Char(size), val)
            },
            Type::Reference => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                let val5 = self.operand_stack.pop().expect("Stack underflow");
                let val6 = self.operand_stack.pop().expect("Stack underflow");
                let val7 = self.operand_stack.pop().expect("Stack underflow");
                let val8 = self.operand_stack.pop().expect("Stack underflow");
                (Type::Reference, vec![val8, val7, val6, val5, val4, val3, val2, val1])
            },
        };

        self.operand_stack_types.push(ty);
        self.operand_stack_types.push(ty);
        self.operand_stack.extend_from_slice(&bytes);
        self.operand_stack.extend_from_slice(&bytes);
    }

    pub fn store_local(&mut self, index: u8) {
        let ty = self.operand_stack_types.pop().expect("Stack underflow");
        match ty {
            Type::U8 | Type::I8 => {
                let val = self.operand_stack.pop().expect("Stack underflow");
                self.local_variables[index as usize] = 0;
                self.local_variables[index as usize] = val.to_le_bytes()[0] as usize;
            },
            Type::U16 | Type::I16 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                self.local_variables[index as usize] = 0;
                self.local_variables[index as usize] = u16::from_le_bytes([val2, val1]) as usize;
            },
            Type::U32 | Type::I32 | Type::F32 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                self.local_variables[index as usize] = u32::from_le_bytes([val4, val3, val2, val1]) as usize;
            },
            Type::U64 | Type::I64 | Type::F64 => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                let val5 = self.operand_stack.pop().expect("Stack underflow");
                let val6 = self.operand_stack.pop().expect("Stack underflow");
                let val7 = self.operand_stack.pop().expect("Stack underflow");
                let val8 = self.operand_stack.pop().expect("Stack underflow");
                self.local_variables[index as usize] = u64::from_le_bytes([val8, val7, val6, val5, val4, val3, val2, val1]) as usize;
            },
            Type::Char(size) => {
                let mut val = Vec::new();
                for _ in 0..size {
                    val.push(self.operand_stack.pop().expect("Stack underflow"));
                }
                self.local_variables[index as usize] = 0;
                self.local_variables[index as usize] = val.into_iter().fold(0, |acc, x| acc << 8 | x as usize);
            },
            Type::Reference => {
                let val1 = self.operand_stack.pop().expect("Stack underflow");
                let val2 = self.operand_stack.pop().expect("Stack underflow");
                let val3 = self.operand_stack.pop().expect("Stack underflow");
                let val4 = self.operand_stack.pop().expect("Stack underflow");
                let val5 = self.operand_stack.pop().expect("Stack underflow");
                let val6 = self.operand_stack.pop().expect("Stack underflow");
                let val7 = self.operand_stack.pop().expect("Stack underflow");
                let val8 = self.operand_stack.pop().expect("Stack underflow");
                self.local_variables[index as usize] = 0;
                self.local_variables[index as usize] = u64::from_le_bytes([val8, val7, val6, val5, val4, val3, val2, val1]) as usize;
            },
        }
        self.local_variable_types[index as usize] = ty;
    }

    pub fn load_local(&mut self, index: u8) {
        let ty = self.local_variable_types[index as usize];
        match ty {
            Type::U8 | Type::I8 => {
                let val = self.local_variables[index as usize];
                self.operand_stack.push(val as u8);
            },
            Type::U16 | Type::I16 => {
                let val = self.local_variables[index as usize] as u16;
                self.operand_stack.push(val.to_le_bytes()[0]);
                self.operand_stack.push(val.to_le_bytes()[1]);
            },
            Type::U32 | Type::I32 | Type::F32 => {
                let val = self.local_variables[index as usize] as u32;
                self.operand_stack.push(val.to_le_bytes()[0]);
                self.operand_stack.push(val.to_le_bytes()[1]);
                self.operand_stack.push(val.to_le_bytes()[2]);
                self.operand_stack.push(val.to_le_bytes()[3]);
            },
            Type::U64 | Type::I64 | Type::F64 => {
                let val = self.local_variables[index as usize] as u64;
                self.operand_stack.push(val.to_le_bytes()[0]);
                self.operand_stack.push(val.to_le_bytes()[1]);
                self.operand_stack.push(val.to_le_bytes()[2]);
                self.operand_stack.push(val.to_le_bytes()[3]);
                self.operand_stack.push(val.to_le_bytes()[4]);
                self.operand_stack.push(val.to_le_bytes()[5]);
                self.operand_stack.push(val.to_le_bytes()[6]);
                self.operand_stack.push(val.to_le_bytes()[7]);
            },
            Type::Char(size) => {
                let val = self.local_variables[index as usize];
                let mut val = val;
                let mut vec = Vec::new();
                for _ in 0..size {
                    vec.push(val as u8);
                    val >>= 8;
                }
                vec.reverse();
                for x in vec {
                    self.operand_stack.push(x);
                }
            },
            Type::Reference => {
                let val = self.local_variables[index as usize] as u64;
                self.operand_stack.push(val.to_le_bytes()[0]);
                self.operand_stack.push(val.to_le_bytes()[1]);
                self.operand_stack.push(val.to_le_bytes()[2]);
                self.operand_stack.push(val.to_le_bytes()[3]);
                self.operand_stack.push(val.to_le_bytes()[4]);
                self.operand_stack.push(val.to_le_bytes()[5]);
                self.operand_stack.push(val.to_le_bytes()[6]);
                self.operand_stack.push(val.to_le_bytes()[7]);
            },
        }
        self.operand_stack_types.push(ty);
    }
}

impl StackFrameUtils<i8> for StackFrame {
    fn push(&mut self, value: i8) {
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack_types.push(Type::I8);
    }

    fn pop(&mut self) -> i8 {
        self.operand_stack_types.pop().expect("Stack underflow");
        i8::from_le_bytes([self.operand_stack.pop().expect("Stack underflow")])
    }

    fn store_argument(&mut self, index: u8, value: i8) {
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = value.to_le_bytes()[0] as usize;
        self.local_variable_types[index as usize] = Type::I8;
    }

    fn load_argument(&mut self, index: u8) -> i8 {
        self.local_variable_types[index as usize] = Type::I8;
        i8::from_le_bytes([self.local_variables[index as usize].to_le_bytes()[0]])
    }
}

impl StackFrameUtils<i16> for StackFrame {
    fn push(&mut self, value: i16) {
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack.push(value.to_le_bytes()[1]);
        self.operand_stack_types.push(Type::I16);
    }

    fn pop(&mut self) -> i16 {
        self.operand_stack_types.pop().expect("Stack underflow");
        let val1 = self.operand_stack.pop().expect("Stack underflow");
        let val2 = self.operand_stack.pop().expect("Stack underflow");
        i16::from_le_bytes([val2, val1])
    }

    fn store_argument(&mut self, index: u8, value: i16) {
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = u16::from_le_bytes([value.to_le_bytes()[0], value.to_le_bytes()[1]]) as usize;
        self.local_variable_types[index as usize] = Type::I16;
    }

    fn load_argument(&mut self, index: u8) -> i16 {
        self.local_variable_types[index as usize] = Type::I16;
        i16::from_le_bytes([self.local_variables[index as usize].to_le_bytes()[0], self.local_variables[index as usize].to_le_bytes()[1]])
    }
}

impl StackFrameUtils<i32> for StackFrame {
    fn push(&mut self, value: i32) {
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack.push(value.to_le_bytes()[1]);
        self.operand_stack.push(value.to_le_bytes()[2]);
        self.operand_stack.push(value.to_le_bytes()[3]);
        self.operand_stack_types.push(Type::I32);
    }

    fn pop(&mut self) -> i32 {
        self.operand_stack_types.pop().expect("Stack underflow");
        let val1 = self.operand_stack.pop().expect("Stack underflow");
        let val2 = self.operand_stack.pop().expect("Stack underflow");
        let val3 = self.operand_stack.pop().expect("Stack underflow");
        let val4 = self.operand_stack.pop().expect("Stack underflow");
        i32::from_le_bytes([val4, val3, val2, val1])
    }

    fn store_argument(&mut self, index: u8, value: i32) {
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = i32::from_le_bytes([value.to_le_bytes()[0], value.to_le_bytes()[1], value.to_le_bytes()[2], value.to_le_bytes()[3]]) as usize;
        self.local_variable_types[index as usize] = Type::I32;
    }

    fn load_argument(&mut self, index: u8) -> i32 {
        self.local_variable_types[index as usize] = Type::I32;
        i32::from_le_bytes([self.local_variables[index as usize].to_le_bytes()[0], self.local_variables[index as usize].to_le_bytes()[1], self.local_variables[index as usize].to_le_bytes()[2], self.local_variables[index as usize].to_le_bytes()[3]])
    }
}

impl StackFrameUtils<i64> for StackFrame {
    fn push(&mut self, value: i64) {
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack.push(value.to_le_bytes()[1]);
        self.operand_stack.push(value.to_le_bytes()[2]);
        self.operand_stack.push(value.to_le_bytes()[3]);
        self.operand_stack.push(value.to_le_bytes()[4]);
        self.operand_stack.push(value.to_le_bytes()[5]);
        self.operand_stack.push(value.to_le_bytes()[6]);
        self.operand_stack.push(value.to_le_bytes()[7]);
        self.operand_stack_types.push(Type::I64);
    }

    fn pop(&mut self) -> i64 {
        self.operand_stack_types.pop().expect("Stack underflow");
        let val1 = self.operand_stack.pop().expect("Stack underflow");
        let val2 = self.operand_stack.pop().expect("Stack underflow");
        let val3 = self.operand_stack.pop().expect("Stack underflow");
        let val4 = self.operand_stack.pop().expect("Stack underflow");
        let val5 = self.operand_stack.pop().expect("Stack underflow");
        let val6 = self.operand_stack.pop().expect("Stack underflow");
        let val7 = self.operand_stack.pop().expect("Stack underflow");
        let val8 = self.operand_stack.pop().expect("Stack underflow");
        i64::from_le_bytes([val8, val7, val6, val5, val4, val3, val2, val1])
    }

    fn store_argument(&mut self, index: u8, value: i64) {
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = u64::from_le_bytes([value.to_le_bytes()[0], value.to_le_bytes()[1], value.to_le_bytes()[2], value.to_le_bytes()[3], value.to_le_bytes()[4], value.to_le_bytes()[5], value.to_le_bytes()[6], value.to_le_bytes()[7]]) as usize;
        self.local_variable_types[index as usize] = Type::I64;
    }

    fn load_argument(&mut self, index: u8) -> i64 {
        self.local_variable_types[index as usize] = Type::I64;
        i64::from_le_bytes([self.local_variables[index as usize].to_le_bytes()[0], self.local_variables[index as usize].to_le_bytes()[1], self.local_variables[index as usize].to_le_bytes()[2], self.local_variables[index as usize].to_le_bytes()[3], self.local_variables[index as usize].to_le_bytes()[4], self.local_variables[index as usize].to_le_bytes()[5], self.local_variables[index as usize].to_le_bytes()[6], self.local_variables[index as usize].to_le_bytes()[7]])
    }
}

impl StackFrameUtils<u8> for StackFrame {
    fn push(&mut self, value: u8) {
        self.operand_stack.push(value);
        self.operand_stack_types.push(Type::U8);
    }

    fn pop(&mut self) -> u8 {
        self.operand_stack_types.pop().expect("Stack underflow");
        self.operand_stack.pop().expect("Stack underflow")
    }

    fn store_argument(&mut self, index: u8, value: u8) {
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = value as usize;
        self.local_variable_types[index as usize] = Type::U8;
    }

    fn load_argument(&mut self, index: u8) -> u8 {
        self.local_variable_types[index as usize] = Type::U8;
        self.local_variables[index as usize] as u8
    }
}

impl StackFrameUtils<u16> for StackFrame {
    fn push(&mut self, value: u16) {
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack.push(value.to_le_bytes()[1]);
        self.operand_stack_types.push(Type::U16);
    }

    fn pop(&mut self) -> u16 {
        self.operand_stack_types.pop().expect("Stack underflow");
        let val1 = self.operand_stack.pop().expect("Stack underflow");
        let val2 = self.operand_stack.pop().expect("Stack underflow");
        u16::from_le_bytes([val2, val1])
    }

    fn store_argument(&mut self, index: u8, value: u16) {
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = value as usize;
        self.local_variable_types[index as usize] = Type::U16;
    }

    fn load_argument(&mut self, index: u8) -> u16 {
        self.local_variable_types[index as usize] = Type::U16;
        self.local_variables[index as usize] as u16
    }
}

impl StackFrameUtils<u32> for StackFrame {
    fn push(&mut self, value: u32) {
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack.push(value.to_le_bytes()[1]);
        self.operand_stack.push(value.to_le_bytes()[2]);
        self.operand_stack.push(value.to_le_bytes()[3]);
        self.operand_stack_types.push(Type::U32);
    }

    fn pop(&mut self) -> u32 {
        self.operand_stack_types.pop().expect("Stack underflow");
        let val1 = self.operand_stack.pop().expect("Stack underflow");
        let val2 = self.operand_stack.pop().expect("Stack underflow");
        let val3 = self.operand_stack.pop().expect("Stack underflow");
        let val4 = self.operand_stack.pop().expect("Stack underflow");
        u32::from_le_bytes([val4, val3, val2, val1])
    }

    fn store_argument(&mut self, index: u8, value: u32) {
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = value as usize;
        self.local_variable_types[index as usize] = Type::U32;
    }

    fn load_argument(&mut self, index: u8) -> u32 {
        self.local_variable_types[index as usize] = Type::U32;
        self.local_variables[index as usize] as u32
    }
}

impl StackFrameUtils<u64> for StackFrame {
    fn push(&mut self, value: u64) {
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack.push(value.to_le_bytes()[1]);
        self.operand_stack.push(value.to_le_bytes()[2]);
        self.operand_stack.push(value.to_le_bytes()[3]);
        self.operand_stack.push(value.to_le_bytes()[4]);
        self.operand_stack.push(value.to_le_bytes()[5]);
        self.operand_stack.push(value.to_le_bytes()[6]);
        self.operand_stack.push(value.to_le_bytes()[7]);
        self.operand_stack_types.push(Type::U64);
    }

    fn pop(&mut self) -> u64 {
        self.operand_stack_types.pop().expect("Stack underflow");
        let val1 = self.operand_stack.pop().expect("Stack underflow");
        let val2 = self.operand_stack.pop().expect("Stack underflow");
        let val3 = self.operand_stack.pop().expect("Stack underflow");
        let val4 = self.operand_stack.pop().expect("Stack underflow");
        let val5 = self.operand_stack.pop().expect("Stack underflow");
        let val6 = self.operand_stack.pop().expect("Stack underflow");
        let val7 = self.operand_stack.pop().expect("Stack underflow");
        let val8 = self.operand_stack.pop().expect("Stack underflow");
        u64::from_le_bytes([val8, val7, val6, val5, val4, val3, val2, val1])
    }

    fn store_argument(&mut self, index: u8, value: u64) {
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = value as usize;
        self.local_variable_types[index as usize] = Type::U64;
    }

    fn load_argument(&mut self, index: u8) -> u64 {
        self.local_variable_types[index as usize] = Type::U64;
        self.local_variables[index as usize] as u64
    }
}

impl StackFrameUtils<f32> for StackFrame {
    fn push(&mut self, value: f32) {
        let value = value.to_bits();
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack.push(value.to_le_bytes()[1]);
        self.operand_stack.push(value.to_le_bytes()[2]);
        self.operand_stack.push(value.to_le_bytes()[3]);
        self.operand_stack_types.push(Type::F32);
    }

    fn pop(&mut self) -> f32 {
        self.operand_stack_types.pop().expect("Stack underflow");
        let val1 = self.operand_stack.pop().expect("Stack underflow");
        let val2 = self.operand_stack.pop().expect("Stack underflow");
        let val3 = self.operand_stack.pop().expect("Stack underflow");
        let val4 = self.operand_stack.pop().expect("Stack underflow");
        f32::from_bits(u32::from_le_bytes([val4, val3, val2, val1]))
    }

    fn store_argument(&mut self, index: u8, value: f32) {
        let value = value.to_bits();
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = value as usize;
        self.local_variable_types[index as usize] = Type::F32;
    }

    fn load_argument(&mut self, index: u8) -> f32 {
        self.local_variable_types[index as usize] = Type::F32;
        f32::from_bits(self.local_variables[index as usize] as u32)
    }
}

impl StackFrameUtils<f64> for StackFrame {
    fn push(&mut self, value: f64) {
        let value = value.to_bits();
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack.push(value.to_le_bytes()[1]);
        self.operand_stack.push(value.to_le_bytes()[2]);
        self.operand_stack.push(value.to_le_bytes()[3]);
        self.operand_stack.push(value.to_le_bytes()[4]);
        self.operand_stack.push(value.to_le_bytes()[5]);
        self.operand_stack.push(value.to_le_bytes()[6]);
        self.operand_stack.push(value.to_le_bytes()[7]);
        self.operand_stack_types.push(Type::F64);
    }

    fn pop(&mut self) -> f64 {
        self.operand_stack_types.pop().expect("Stack underflow");
        let val1 = self.operand_stack.pop().expect("Stack underflow");
        let val2 = self.operand_stack.pop().expect("Stack underflow");
        let val3 = self.operand_stack.pop().expect("Stack underflow");
        let val4 = self.operand_stack.pop().expect("Stack underflow");
        let val5 = self.operand_stack.pop().expect("Stack underflow");
        let val6 = self.operand_stack.pop().expect("Stack underflow");
        let val7 = self.operand_stack.pop().expect("Stack underflow");
        let val8 = self.operand_stack.pop().expect("Stack underflow");
        f64::from_bits(u64::from_le_bytes([val8, val7, val6, val5, val4, val3, val2, val1]))
    }

    fn store_argument(&mut self, index: u8, value: f64) {
        let value = value.to_bits();
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = value as usize;
        self.local_variable_types[index as usize] = Type::F64;
    }

    fn load_argument(&mut self, index: u8) -> f64 {
        self.local_variable_types[index as usize] = Type::F64;
        f64::from_bits(self.local_variables[index as usize] as u64)
    }
}

impl StackFrameUtils<Reference> for StackFrame {
    fn push(&mut self, value: Reference) {
        self.operand_stack.push(value.to_le_bytes()[0]);
        self.operand_stack.push(value.to_le_bytes()[1]);
        self.operand_stack.push(value.to_le_bytes()[2]);
        self.operand_stack.push(value.to_le_bytes()[3]);
        self.operand_stack.push(value.to_le_bytes()[4]);
        self.operand_stack.push(value.to_le_bytes()[5]);
        self.operand_stack.push(value.to_le_bytes()[6]);
        self.operand_stack.push(value.to_le_bytes()[7]);
        self.operand_stack_types.push(Type::Reference);
    }

    fn pop(&mut self) -> Reference {
        self.operand_stack_types.pop().expect("Stack underflow");
        let val1 = self.operand_stack.pop().expect("Stack underflow");
        let val2 = self.operand_stack.pop().expect("Stack underflow");
        let val3 = self.operand_stack.pop().expect("Stack underflow");
        let val4 = self.operand_stack.pop().expect("Stack underflow");
        let val5 = self.operand_stack.pop().expect("Stack underflow");
        let val6 = self.operand_stack.pop().expect("Stack underflow");
        let val7 = self.operand_stack.pop().expect("Stack underflow");
        let val8 = self.operand_stack.pop().expect("Stack underflow");
        u64::from_le_bytes([val8, val7, val6, val5, val4, val3, val2, val1]) as Reference
    }

    fn store_argument(&mut self, index: u8, value: Reference) {
        self.local_variables[index as usize] = 0;
        self.local_variables[index as usize] = value as usize;
        self.local_variable_types[index as usize] = Type::Reference;
    }

    fn load_argument(&mut self, index: u8) -> Reference {
        self.local_variable_types[index as usize] = Type::Reference;
        self.local_variables[index as usize] as Reference
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_i8() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1i8);
        assert_eq!(StackFrameUtils::<i8>::pop(&mut stack_frame), 1i8);
    }

    #[test]
    fn test_i16() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1i16);
        assert_eq!(StackFrameUtils::<i16>::pop(&mut stack_frame), 1i16);
    }

    #[test]
    fn test_i32() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1i32);
        assert_eq!(StackFrameUtils::<i32>::pop(&mut stack_frame), 1i32);
    }

    #[test]
    fn test_i64() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1i64);
        assert_eq!(StackFrameUtils::<i64>::pop(&mut stack_frame), 1i64);
    }

    #[test]
    fn test_u8() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1u8);
        assert_eq!(StackFrameUtils::<u8>::pop(&mut stack_frame), 1u8);
    }

    #[test]
    fn test_u16() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1u16);
        assert_eq!(StackFrameUtils::<u16>::pop(&mut stack_frame), 1u16);
    }

    #[test]
    fn test_u32() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1u32);
        assert_eq!(StackFrameUtils::<u32>::pop(&mut stack_frame), 1u32);
    }

    #[test]
    fn test_u64() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1u64);
        assert_eq!(StackFrameUtils::<u64>::pop(&mut stack_frame), 1u64);
    }

    #[test]
    fn test_f32() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1.0f32);
        assert_eq!(StackFrameUtils::<f32>::pop(&mut stack_frame), 1.0f32);
    }

    #[test]
    fn test_f64() {
        let mut stack_frame = StackFrame::new(0, 0);
        stack_frame.push(1.0f64);
        assert_eq!(StackFrameUtils::<f64>::pop(&mut stack_frame), 1.0f64);
    }

    #[test]
    fn test_reference() {
        let mut stack_frame = StackFrame::new(0, 0);
        let refe: usize = 1;
        stack_frame.push(refe);
        assert_eq!(StackFrameUtils::<Reference>::pop(&mut stack_frame), 1);
    }

    #[test]
    fn test_i8_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<i8>::store_argument(&mut stack_frame, 0, 1);
        assert_eq!(StackFrameUtils::<i8>::load_argument(&mut stack_frame, 0), 1);
    }

    #[test]
    fn test_i16_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<i16>::store_argument(&mut stack_frame, 0, 1);
        assert_eq!(StackFrameUtils::<i16>::load_argument(&mut stack_frame, 0), 1);
    }

    #[test]
    fn test_i32_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<i32>::store_argument(&mut stack_frame, 0, 1);
        assert_eq!(StackFrameUtils::<i32>::load_argument(&mut stack_frame, 0), 1);
    }

    #[test]
    fn test_i64_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<i64>::store_argument(&mut stack_frame, 0, 1);
        assert_eq!(StackFrameUtils::<i64>::load_argument(&mut stack_frame, 0), 1);
    }

    #[test]
    fn test_u8_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<u8>::store_argument(&mut stack_frame, 0, 1);
        assert_eq!(StackFrameUtils::<u8>::load_argument(&mut stack_frame, 0), 1);
    }

    #[test]
    fn test_u16_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<u16>::store_argument(&mut stack_frame, 0, 1);
        assert_eq!(StackFrameUtils::<u16>::load_argument(&mut stack_frame, 0), 1);
    }

    #[test]
    fn test_u32_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<u32>::store_argument(&mut stack_frame, 0, 1);
        assert_eq!(StackFrameUtils::<u32>::load_argument(&mut stack_frame, 0), 1);
    }

    #[test]
    fn test_u64_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<u64>::store_argument(&mut stack_frame, 0, 1);
        assert_eq!(StackFrameUtils::<u64>::load_argument(&mut stack_frame, 0), 1);
    }

    #[test]
    fn test_f32_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<f32>::store_argument(&mut stack_frame, 0, 1.0);
        assert_eq!(StackFrameUtils::<f32>::load_argument(&mut stack_frame, 0), 1.0);
    }

    #[test]
    fn test_f64_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<f64>::store_argument(&mut stack_frame, 0, 1.0);
        assert_eq!(StackFrameUtils::<f64>::load_argument(&mut stack_frame, 0), 1.0);
    }

    #[test]
    fn test_reference_argument() {
        let mut stack_frame = StackFrame::new(0, 0);
        StackFrameUtils::<Reference>::store_argument(&mut stack_frame, 0, 1);
        assert_eq!(StackFrameUtils::<Reference>::load_argument(&mut stack_frame, 0), 1);
    }

    
    

}
