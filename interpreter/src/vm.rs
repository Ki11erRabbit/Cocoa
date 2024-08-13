use bytecode::Bytecode;
use cranelift_module::{DataDescription, Linkage, Module};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};


#[derive(Debug, Clone, Copy)]
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
    builder_context: FunctionBuilderContext,
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
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_description: DataDescription::new(),
            module,
        }
    }

    pub fn run(&mut self) {
        let code = self.compile().unwrap();

        unsafe {
            let func: fn() -> i64 = std::mem::transmute(code);
            let _ = func();
        }

        
    }

    fn compile(&mut self) -> Result<*const u8, String> {

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let mut trans = FunctionTranslator::new(builder, &mut self.module, &self.constants, &self.bytecode);

        trans.translate();

        trans.builder.finalize();

        let id = self
            .module
            .declare_function("main", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;

        self.module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;

        self.module.clear_context(&mut self.ctx);

        self.module.finalize_definitions().unwrap();

        let code = self.module.get_finalized_function(id);

        Ok(code)
    }
}


pub struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    module: &'a mut JITModule,
    constants: &'a ConstantPool,
    bytecode: &'a Vec<Bytecode>,
    locals: [Value; 256],
    stack: Vec<Value>,
}

impl<'a> FunctionTranslator<'a> {

    pub fn new(builder: FunctionBuilder<'a>, module: &'a mut JITModule, constants: &'a ConstantPool, bytecode: &'a Vec<Bytecode>) -> Self {
        Self {
            builder,
            module,
            constants,
            bytecode,
            locals: [Value::from_u32(0); 256],
            stack: Vec::new(),
        }
    }

    pub fn translate(&mut self) {
        for code in self.bytecode {
            match code {
                Bytecode::LoadConstant(pos) => {
                    let constant = self.constants.constants[*pos as usize];
                    match constant {
                        Constant::U8(val) => {
                            let slice = [val.to_le_bytes()[0], 0, 0, 0, 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I8, i64::from_le_bytes(slice));
                            self.stack.push(val);
                        }
                        Constant::U16(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], 0, 0, 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I16, i64::from_le_bytes(slice));
                            self.stack.push(val);
                        }
                        Constant::U32(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], val.to_le_bytes()[2], val.to_le_bytes()[3], 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I32, i64::from_le_bytes(slice));
                            self.stack.push(val);
                        }
                        Constant::U64(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], val.to_le_bytes()[2], val.to_le_bytes()[3], val.to_le_bytes()[4], val.to_le_bytes()[5], val.to_le_bytes()[6], val.to_le_bytes()[7]];
                            let val = self.builder.ins().iconst(types::I64, i64::from_le_bytes(slice));
                            self.stack.push(val);
                        }
                        Constant::I8(val) => {
                            let slice = [val.to_le_bytes()[0], 0, 0, 0, 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I8, i64::from_le_bytes(slice));
                            self.stack.push(val);
                        }
                        Constant::I16(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], 0, 0, 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I16, i64::from_le_bytes(slice));
                            self.stack.push(val);
                        }
                        Constant::I32(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], val.to_le_bytes()[2], val.to_le_bytes()[3], 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I32, i64::from_le_bytes(slice));
                            self.stack.push(val);
                        }
                        Constant::I64(val) => {
                            let val = self.builder.ins().iconst(types::I64, val);
                            self.stack.push(val);
                        }
                        Constant::F32(val) => {
                            let val = self.builder.ins().f32const(val);
                            self.stack.push(val);
                        }
                        Constant::F64(val) => {
                            let val = self.builder.ins().f64const(val);
                            self.stack.push(val);
                        }
                        Constant::Char(_) => {
                            unimplemented!();
                        }
                    }
                }
                Bytecode::StoreConstant(_) => {
                    unimplemented!();
                }
                Bytecode::Pop => {
                    self.stack.pop();
                }
                Bytecode::Dup => {
                    let val = self.stack.last().unwrap();
                    self.stack.push(*val);
                }
                Bytecode::Swap => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    self.stack.push(val1);
                    self.stack.push(val2);
                }
                Bytecode::StoreLocal(index) => {
                    let val = self.stack.pop().unwrap();
                    self.locals[*index as usize] = val;
                }
                Bytecode::LoadLocal(index) => {
                    let val = self.locals[*index as usize];
                    self.stack.push(val);
                }
                Bytecode::StoreArgument => {
                    unimplemented!();
                }
                Bytecode::Addu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().iadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Addu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().iadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Addu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().iadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Addu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().iadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Addi8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().iadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Addi16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().iadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Addi32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().iadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Addi64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().iadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().isub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().isub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().isub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().isub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subi8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().isub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subi16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().isub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subi32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().isub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subi64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().isub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Mulu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().imul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Mulu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().imul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Mulu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().imul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Mulu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().imul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Muli8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().imul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Muli16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().imul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Muli32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().imul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Muli64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().imul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().udiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().udiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().udiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().udiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divi8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().sdiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divi16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().sdiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divi32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().sdiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divi64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().sdiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Modu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().urem(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Modu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().urem(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Modu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().urem(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Modu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().urem(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Modi8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().srem(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Modi16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().srem(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Modi32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().srem(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Modi64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().srem(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Andu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().band(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Andu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().band(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Andu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().band(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Andu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().band(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Andi8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().band(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Andi16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().band(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Andi32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().band(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Andi64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().band(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Oru8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Oru16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Oru32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Oru64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Ori8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Ori16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Ori32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Ori64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Xoru8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bxor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Xoru16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bxor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Xoru32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bxor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Xoru64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bxor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Xori8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bxor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Xori16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bxor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Xori32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bxor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Xori64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().bxor(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Notu8 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().bnot(val);
                    self.stack.push(val);
                }
                Bytecode::Notu16 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().bnot(val);
                    self.stack.push(val);
                }
                Bytecode::Notu32 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().bnot(val);
                    self.stack.push(val);
                }
                Bytecode::Notu64 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().bnot(val);
                    self.stack.push(val);
                }
                Bytecode::Noti8 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().bnot(val);
                    self.stack.push(val);
                }
                Bytecode::Noti16 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().bnot(val);
                    self.stack.push(val);
                }
                Bytecode::Noti32 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().bnot(val);
                    self.stack.push(val);
                }
                Bytecode::Noti64 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().bnot(val);
                    self.stack.push(val);
                }
                Bytecode::Shlu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ishl(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shlu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ishl(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shlu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ishl(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shlu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ishl(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shli8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ishl(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shli16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ishl(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shli32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ishl(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shli64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ishl(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shru8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ushr(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shru16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ushr(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shru32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ushr(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shru64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().ushr(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shri8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().sshr(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shri16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().sshr(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shri32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().sshr(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Shri64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().sshr(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Addf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Addf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fadd(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fsub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Subf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fsub(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Mulf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fmul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Mulf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fmul(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fdiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Divf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fdiv(val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Negu8 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().ineg(val);
                    self.stack.push(val);
                }
                Bytecode::Negu16 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().ineg(val);
                    self.stack.push(val);
                }
                Bytecode::Negu32 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().ineg(val);
                    self.stack.push(val);
                }
                Bytecode::Negu64 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().ineg(val);
                    self.stack.push(val);
                }
                Bytecode::Negi8 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().ineg(val);
                    self.stack.push(val);
                }
                Bytecode::Negi16 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().ineg(val);
                    self.stack.push(val);
                }
                Bytecode::Negi32 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().ineg(val);
                    self.stack.push(val);
                }
                Bytecode::Negi64 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().ineg(val);
                    self.stack.push(val);
                }
                Bytecode::Negf32 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().fneg(val);
                    self.stack.push(val);
                }
                Bytecode::Negf64 => {
                    let val = self.stack.pop().unwrap();
                    let val = self.builder.ins().fneg(val);
                    self.stack.push(val);
                }
                Bytecode::Return => {
                    let val = self.stack.pop().unwrap();
                    self.builder.ins().return_(&[val]);
                }
                Bytecode::ReturnUnit => {
                    self.builder.ins().return_(&[]);
                }
                _ => {
                    unimplemented!();
                }
                
                
                
                
                
                
            }
        }
    }
}
