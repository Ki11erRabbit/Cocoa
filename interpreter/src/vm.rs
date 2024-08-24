mod function_translator;


use definitions::bytecode::Bytecode;
use cranelift_module::{DataDescription, Linkage, Module};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};

use crate::vm::function_translator::FunctionTranslator;




pub struct Jit<'a> {
    module: &'a definitions::Module,
    bytecode: Vec<Bytecode>,
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_description: DataDescription,
    jit_module: JITModule,
}

impl<'a> Jit<'a> {
    pub fn new(module: &'a definitions::Module, bytecode: Vec<Bytecode>) -> Self {
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
            bytecode,
            builder_context: FunctionBuilderContext::new(),
            ctx: jit_module.make_context(),
            data_description: DataDescription::new(),
            jit_module,
        }
    }

    pub fn run(&mut self, block_count: u64) {
        let code = self.compile(block_count).unwrap();

        println!("Running code...");
        unsafe {
            let func: fn() -> i64 = std::mem::transmute(code);
            let res = func();
            println!("{}", res);
        }

        
    }

    fn compile(&mut self, block_count: u64) -> Result<*const u8, String> {

        self.ctx.func.signature.returns.push(AbiParam::new(types::I64));

        let builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        let mut trans = FunctionTranslator::new(builder, &mut self.jit_module, &self.module, block_count);

        trans.translate(self.bytecode.iter());

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
            .declare_function("main", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;

        self.jit_module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;

        self.jit_module.clear_context(&mut self.ctx);

        self.jit_module.finalize_definitions().unwrap();

        let code = self.jit_module.get_finalized_function(id);

        Ok(code)
    }
}


