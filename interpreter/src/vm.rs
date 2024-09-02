mod function_translator;


use definitions::{bytecode::Bytecode, module::{constants, functions}};
use cranelift_module::{DataDescription, Linkage, Module};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};

use crate::vm::function_translator::FunctionTranslator;




pub struct Jit<'a> {
    module: &'a definitions::Module,
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_description: DataDescription,
    jit_module: JITModule,
}

impl<'a> Jit<'a> {
    pub fn new(module: &'a definitions::Module) -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("error creating Cranelift native builder: {}", msg);
        });
        let isa = isa_builder.finish(settings::Flags::new(flag_builder)).unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let jit_module = JITModule::new(builder);
        Self {
            module,
            builder_context: FunctionBuilderContext::new(),
            ctx: jit_module.make_context(),
            data_description: DataDescription::new(),
            jit_module,
        }
    }

    pub fn run(&mut self) {
        let code = self.compile().unwrap();

        println!("Running code...");
        unsafe {
            let func: fn() -> i64 = std::mem::transmute(code);
            let res = func();
            println!("{}", res);
        }

        
    }

    fn compile(&mut self) -> Result<*const u8, String> {

        let mut main_function = None;

        for function in self.module.function_table.iter() {
            let functions::Function { name_symbol, block_count, byte_code, .. } = function;

            let (name, type_info) = match &self.module.constant_pool[*name_symbol] {
                constants::Constant::Symbol(x) => (&x.name, &x.type_info),
                _ => panic!("Invalid constant type"),
            };
            
            println!("Compiling function: {}", name);

            match type_info {
                constants::TypeInfo::Simple(_) => panic!("Expected function type"),
                constants::TypeInfo::Function(args_types, return_type) => {
                    let mut sig = Signature::new(isa::CallConv::SystemV);
                    for arg_type in args_types.iter() {
                        match arg_type {
                            constants::TypeInfo::Simple(constants::Type::U8) => sig.params.push(AbiParam::new(types::I8)),
                            constants::TypeInfo::Simple(constants::Type::U16) => sig.params.push(AbiParam::new(types::I16)),
                            constants::TypeInfo::Simple(constants::Type::U32) => sig.params.push(AbiParam::new(types::I32)),
                            constants::TypeInfo::Simple(constants::Type::U64) => sig.params.push(AbiParam::new(types::I64)),
                            constants::TypeInfo::Simple(constants::Type::I8) => sig.params.push(AbiParam::new(types::I8)),
                            constants::TypeInfo::Simple(constants::Type::I16) => sig.params.push(AbiParam::new(types::I16)),
                            constants::TypeInfo::Simple(constants::Type::I32) => sig.params.push(AbiParam::new(types::I32)),
                            constants::TypeInfo::Simple(constants::Type::I64) => sig.params.push(AbiParam::new(types::I64)),
                            constants::TypeInfo::Simple(constants::Type::F32) => sig.params.push(AbiParam::new(types::F32)),
                            constants::TypeInfo::Simple(constants::Type::F64) => sig.params.push(AbiParam::new(types::F64)),
                            _ => unimplemented!(),
                        }
                    }

                    match return_type.as_ref() {
                        constants::TypeInfo::Simple(constants::Type::U8) => sig.returns.push(AbiParam::new(types::I8)),
                        constants::TypeInfo::Simple(constants::Type::U16) => sig.returns.push(AbiParam::new(types::I16)),
                        constants::TypeInfo::Simple(constants::Type::U32) => sig.returns.push(AbiParam::new(types::I32)),
                        constants::TypeInfo::Simple(constants::Type::U64) => sig.returns.push(AbiParam::new(types::I64)),
                        constants::TypeInfo::Simple(constants::Type::I8) => sig.returns.push(AbiParam::new(types::I8)),
                        constants::TypeInfo::Simple(constants::Type::I16) => sig.returns.push(AbiParam::new(types::I16)),
                        constants::TypeInfo::Simple(constants::Type::I32) => sig.returns.push(AbiParam::new(types::I32)),
                        constants::TypeInfo::Simple(constants::Type::I64) => sig.returns.push(AbiParam::new(types::I64)),
                        constants::TypeInfo::Simple(constants::Type::F32) => sig.returns.push(AbiParam::new(types::F32)),
                        constants::TypeInfo::Simple(constants::Type::F64) => sig.returns.push(AbiParam::new(types::F64)),
                        _ => unimplemented!(),
                    }

                    self.ctx.func.signature = sig;
                }
            }

            let builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
            let mut trans = FunctionTranslator::new(builder, &mut self.jit_module, &self.module, *block_count);

            trans.translate(byte_code.iter());

            let flags = settings::Flags::new(settings::builder());
            let res = trans.verify(&flags);
            trans.display_ir();
            if let Err(errors) = res {
                println!("{}", errors);
                panic!("Verification failed");
            }

            trans.finalize();

            let id = self
                .jit_module
                .declare_function(&name, Linkage::Export, &self.ctx.func.signature)
                .map_err(|e| e.to_string())?;

            self.jit_module
                .define_function(id, &mut self.ctx)
                .map_err(|e| e.to_string())?;

            
            self.jit_module.clear_context(&mut self.ctx);

            if name == "main" {
                main_function = Some(id);
            }
            
        }

        self.jit_module.finalize_definitions().unwrap();

        let id = main_function.ok_or("No main function found".to_string())?;

        let code = self.jit_module.get_finalized_function(id);

        Ok(code)
    }
}


