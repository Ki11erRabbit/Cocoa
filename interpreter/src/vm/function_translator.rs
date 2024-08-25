use std::collections::HashMap;

use cranelift::{codegen::{verifier::VerifierErrors, verify_function}, prelude::*};
use cranelift_jit::JITModule;
use definitions::{bytecode::Bytecode, module::constants::Constant, Module};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
}

impl From<u8> for Type {
    fn from(val: u8) -> Self {
        match val {
            0 => Type::U8,
            1 => Type::U16,
            2 => Type::U32,
            3 => Type::U64,
            4 => Type::I8,
            5 => Type::I16,
            6 => Type::I32,
            7 => Type::I64,
            8 => Type::F32,
            9 => Type::F64,
            10 => Type::Char,
            _ => panic!("Invalid type"),
        }
    }
}




pub struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    jit_module: &'a mut JITModule,
    module: &'a Module,
    locals: Vec<Variable>,
    locals_types: Vec<types::Type>,
    stack: Vec<Value>,
    stack_types: Vec<types::Type>,
    arguments: Vec<Value>,
    next_variable: usize,
    blocks: Vec<Block>,
    current_block: usize,
    block_arg_types: HashMap<u64, Vec<types::Type>>,
}

impl<'a> FunctionTranslator<'a> {

    pub fn new(
        mut builder: FunctionBuilder<'a>,
        jit_module: &'a mut JITModule,
        module: &'a Module,
        block_count: u64
    ) -> Self {
        let mut locals = Vec::with_capacity(256);
        let mut locals_types = Vec::with_capacity(256);
        for i in 0..256 {
            locals.push(Variable::new(i));
            locals_types.push(types::I64);
        }
        let mut blocks = Vec::new();
        for _ in 0..=block_count {
            blocks.push(builder.create_block());
        }
        Self {
            builder,
            jit_module,
            module,
            locals,
            locals_types,
            stack: Vec::new(),
            stack_types: Vec::new(),
            arguments: Vec::new(),
            next_variable: 0,
            blocks,
            current_block: 0,
            block_arg_types: HashMap::new(),
        }
    }

    pub fn verify(&self, flags: &settings::Flags) -> Result<(), VerifierErrors> {
        verify_function(&self.builder.func, flags)
    }

    /// TODO: This function should be able to reset the state of the builder without consuming the whole translator
    pub fn finalize(self) {
        self.builder.finalize();
    }

    pub fn display_ir(&self) {
        println!("IR:\n{}", self.builder.func.display());
    }

    fn get_variable(&mut self) -> Variable {
        let var = Variable::new(self.next_variable);
        self.next_variable += 1;
        var
    }

    pub fn translate(&mut self, bytecode: impl Iterator<Item = &'a Bytecode>) {
        let entry_block = self.blocks[0];
        self.builder.append_block_params_for_function_params(entry_block);
        self.builder.switch_to_block(entry_block);
        self.blocks.push(entry_block);
        // TODO: check type of arguments and load arguments into variables
        self.translate_block(bytecode);
        self.builder.seal_all_blocks();
    }

    fn translate_block(&mut self, bytecode: impl Iterator<Item = &'a Bytecode>) {
        let mut bytecode = bytecode;
        while let Some(code) = bytecode.next() {
            println!("{:?}", code);
            match code {
                Bytecode::LoadConstant(pos) => {
                    let constant = self.constants.constants[*pos as usize];
                    match constant {
                        Constant::U8(val) => {
                            let slice = [val.to_le_bytes()[0], 0, 0, 0, 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I8, i64::from_le_bytes(slice));
                            self.stack.push(val);
                            self.stack_types.push(types::I8);

                        }
                        Constant::U16(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], 0, 0, 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I16, i64::from_le_bytes(slice));
                            self.stack.push(val);
                            self.stack_types.push(types::I16);
                        }
                        Constant::U32(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], val.to_le_bytes()[2], val.to_le_bytes()[3], 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I32, i64::from_le_bytes(slice));
                            self.stack.push(val);
                            self.stack_types.push(types::I32);
                        }
                        Constant::U64(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], val.to_le_bytes()[2], val.to_le_bytes()[3], val.to_le_bytes()[4], val.to_le_bytes()[5], val.to_le_bytes()[6], val.to_le_bytes()[7]];
                            let val = self.builder.ins().iconst(types::I64, i64::from_le_bytes(slice));
                            self.stack.push(val);
                            self.stack_types.push(types::I64);
                        }
                        Constant::I8(val) => {
                            let slice = [val.to_le_bytes()[0], 0, 0, 0, 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I8, i64::from_le_bytes(slice));
                            self.stack.push(val);
                            self.stack_types.push(types::I8);
                        }
                        Constant::I16(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], 0, 0, 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I16, i64::from_le_bytes(slice));
                            self.stack.push(val);
                            self.stack_types.push(types::I16);
                        }
                        Constant::I32(val) => {
                            let slice = [val.to_le_bytes()[0], val.to_le_bytes()[1], val.to_le_bytes()[2], val.to_le_bytes()[3], 0, 0, 0, 0];
                            let val = self.builder.ins().iconst(types::I32, i64::from_le_bytes(slice));
                            self.stack.push(val);
                            self.stack_types.push(types::I32);
                        }
                        Constant::I64(val) => {
                            let val = self.builder.ins().iconst(types::I64, val);
                            self.stack.push(val);
                            self.stack_types.push(types::I64);
                        }
                        Constant::F32(val) => {
                            let val = self.builder.ins().f32const(val);
                            self.stack.push(val);
                            self.stack_types.push(types::F32);
                        }
                        Constant::F64(val) => {
                            let val = self.builder.ins().f64const(val);
                            self.stack.push(val);
                            self.stack_types.push(types::F64);
                        }
                        Constant::Char(_) => {
                            unimplemented!();
                        }
                        x => {
                            todo!("TODO: {:?}", x);
                        }
                    }
                }
                Bytecode::StoreConstant(_) => {
                    unimplemented!();
                }
                Bytecode::Pop => {
                    self.stack.pop();
                    self.stack_types.pop();
                }
                Bytecode::Dup => {
                    let val = self.stack.last().unwrap();
                    self.stack.push(*val);
                    let ty = self.stack_types.last().unwrap();
                    self.stack_types.push(*ty);
                }
                Bytecode::Swap => {
                    let val1 = self.stack.pop().unwrap();
                    let val1_ty = self.stack_types.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val2_ty = self.stack_types.pop().unwrap();
                    self.stack.push(val1);
                    self.stack_types.push(val1_ty);
                    self.stack.push(val2);
                    self.stack_types.push(val2_ty);

                }
                Bytecode::StoreLocal(index, ty) => {
                    let ty = match ty {
                        0 => types::I8,
                        1 => types::I16,
                        2 => types::I32,
                        3 => types::I64,
                        4 => types::I8,
                        5 => types::I16,
                        6 => types::I32,
                        7 => types::I64,
                        8 => types::F32,
                        9 => types::F64,
                        _ => panic!("Invalid type"),
                    };
                    match self.builder.try_declare_var(self.locals[*index as usize], ty) {
                        Ok(_) => {
                            let val = self.stack.pop().unwrap();
                            let val_ty = self.stack_types.pop().unwrap();
                            self.builder.def_var(self.locals[*index as usize], val);
                            self.locals_types[*index as usize] = val_ty;
                        }
                        Err(_) => {
                            let val = self.stack.pop().unwrap();
                            let val_ty = self.stack_types.pop().unwrap();
                            self.builder.def_var(self.locals[*index as usize], val);
                            self.locals_types[*index as usize] = val_ty;
                        }
                    }
                }
                Bytecode::LoadLocal(index) => {
                    let val = self.builder.use_var(self.locals[*index as usize]);
                    self.stack.push(val);
                    let ty = self.locals_types[*index as usize];
                    self.stack_types.push(ty);
                }
                Bytecode::StoreArgument => {
                    let val = self.stack.pop().unwrap();
                    let _ = self.stack_types.pop().unwrap();
                    self.arguments.push(val);

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
                Bytecode::Equalu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Equalu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Equalu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Equalu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Equali8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Equali16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Equali32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Equali64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Equalf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Equalf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::Equal, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greateru8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedGreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greateru16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedGreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greateru32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedGreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greateru64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedGreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greateri8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedGreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greateri16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedGreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greateri32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedGreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greateri64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedGreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greaterf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::GreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Greaterf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::GreaterThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedLessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedLessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedLessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedLessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessi8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedLessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessi16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedLessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessi32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedLessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessi64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedLessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::LessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::Lessf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::LessThan, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEqualu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEqualu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEqualu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEqualu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEquali8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEquali16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEquali32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEquali64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEqualf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::NotEqualf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::NotEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEqualu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedLessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEqualu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedLessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEqualu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedLessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEqualu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedLessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEquali8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedLessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEquali16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedLessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEquali32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedLessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEquali64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedLessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEqualf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::LessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::LessOrEqualf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::LessThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEqualu8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedGreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEqualu16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedGreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEqualu32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedGreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEqualu64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::UnsignedGreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEquali8 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEquali16 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEquali32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEquali64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEqualf32 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::GreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                Bytecode::GreaterOrEqualf64 => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let val = self.builder.ins().fcmp(FloatCC::GreaterThanOrEqual, val1, val2);
                    self.stack.push(val);
                }
                // Implement convertions
                Bytecode::Goto(blockid) => {
                    let block = self.blocks[*blockid as usize];
                    self.builder.ins().jump(block, &self.stack);

                    let block = self.blocks[*blockid as usize];
                    if let Some(_) = self.block_arg_types.get(blockid) {

                    } else {
                        let mut arg_types = Vec::new();
                        for ty in self.stack_types.iter() {
                            self.builder.append_block_param(block, *ty);
                            arg_types.push(*ty);
                        }

                        self.block_arg_types.insert(*blockid, arg_types);
                    }
                    self.stack.clear();
                    self.stack_types.clear();
                }
                // Implement Jump
                Bytecode::If(then_id, else_id) => {
                    let then_block = self.blocks[*then_id as usize];
                    let else_block = self.blocks[*else_id as usize];

                    let val = self.stack.pop().unwrap();
                    let _ = self.stack_types.pop().unwrap();

                    self.builder.ins().brif(val, then_block, &self.stack, else_block, &self.stack);

                    let block = self.blocks[*then_id as usize];
                    if let Some(_) = self.block_arg_types.get(then_id) {

                    } else {
                        let mut arg_types = Vec::new();
                        for ty in self.stack_types.iter() {
                            self.builder.append_block_param(block, *ty);
                            arg_types.push(*ty);
                        }

                        self.block_arg_types.insert(*then_id, arg_types);
                    }
                    let block = self.blocks[*else_id as usize];
                    if let Some(_) = self.block_arg_types.get(else_id) {

                    } else {
                        let mut arg_types = Vec::new();
                        for ty in self.stack_types.iter() {
                            self.builder.append_block_param(block, *ty);
                            arg_types.push(*ty);
                        }

                        self.block_arg_types.insert(*else_id, arg_types);
                    }
                    self.stack.clear();
                    self.stack_types.clear();
                }
                Bytecode::StartBlock(block_id) => {
                    self.current_block = *block_id as usize;
                    let block = self.blocks[self.current_block];
                    self.builder.switch_to_block(block);

                    self.stack.extend_from_slice(self.builder.block_params(block));
                    if let Some(types) = self.block_arg_types.get(block_id) {
                        self.stack_types.extend_from_slice(types)
                    }
                    
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
