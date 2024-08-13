use bytecode::Bytecode;
use cranelift_module::{DataDescription, Module};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};



pub enum Constant {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
}


pub struct ConstantPool {
    constants: Vec<Constant>,
}

impl ConstantPool {

    pub fn from_binary(iter: &mut dyn Iterator<Item = u8>) -> Self {
        let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
        let len = u64::from_le_bytes(slice) as usize;
        let mut count = 0;
        let mut constants = Vec::new();
        while count < len {
            let constant = match iter.next().unwrap() {
                0 => Constant::U8(u8::from_le_bytes([iter.next().unwrap()])),
                1 => Constant::U16(u16::from_le_bytes([iter.next().unwrap(), iter.next().unwrap()])),
                2 => Constant::U32(u32::from_le_bytes([iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()])),
                3 => Constant::U64(u64::from_le_bytes([iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()])),
                4 => Constant::I8(i8::from_le_bytes([iter.next().unwrap()])),
                5 => Constant::I16(i16::from_le_bytes([iter.next().unwrap(), iter.next().unwrap()])),
                6 => Constant::I32(i32::from_le_bytes([iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()])),
                7 => Constant::I64(i64::from_le_bytes([iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()])),
                8 => Constant::F32(f32::from_le_bytes([iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()])),
                9 => Constant::F64(f64::from_le_bytes([iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(),iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()])),
                10 => {
                    let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
                    let len = u64::from_le_bytes(slice) as usize;
                    let mut string = String::new();
                    let mut vec = Vec::new();
                    for _ in 0..len {
                        vec.push(iter.next().unwrap());
                    }
                    string.push_str(&String::from_utf8(vec).unwrap());
                    Constant::Char(string.chars().next().unwrap())
                }
                _ => panic!("Invalid constant type"),
            };
            constants.push(constant);
            count += 1;
        }

        Self {
            constants,
        }

    }
}

pub fn get_bytecode(iter: &mut dyn Iterator<Item = u8>) -> Vec<Bytecode> {
    let mut bytecode = Vec::new();
    let slice = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];

    let len = u64::from_le_bytes(slice) as usize;
    let mut count = 0;
    while count < len {
        let code = Bytecode::from_binary(iter);
        bytecode.push(code);
        count += 1;
    }
    bytecode
}


pub struct Jit {
    constants: ConstantPool,
    bytecode: Vec<Bytecode>,
    ctx: codegen::Context,
    data_description: DataDescription,
    module: JITModule,
}

impl Jit {
    pub fn new(constants: ConstantPool, bytecode: Vec<Bytecode>) -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("error creating Cranelift native builder: {}", msg);
        });
        let isa = isa_builder.finish(settings::Flags::new(flag_builder)).unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);
        Self {
            constants,
            bytecode,
            ctx: module.make_context(),
            data_description: DataDescription::new(),
            module,
        }
    }

    pub fn run(&mut self) {
        self.compile();
    }

    fn compile(&mut self) -> Result<(), String> {

    }
}
