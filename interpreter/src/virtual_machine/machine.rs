use definitions::{bytecode::Bytecode, class::{Method, PoolEntry, PoolIndex}, object::Reference, stack::{Stack, StackUtils}, CocoaResult};

use crate::virtual_machine::{NativeMethod, NativeMethodTable};

use super::ObjectTableSingleton;



pub struct Machine {
    stack: Stack,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            stack: Stack::new(),
        }
    }

    fn get_instruction(&self) -> Bytecode {
        let class_ref = self.stack.get_class_index();
        let pc = self.stack.get_current_pc();
        let method_index = self.stack.get_current_method_index();

        let object_table = ObjectTableSingleton::get_singleton();

        let class = object_table.get_class(class_ref);

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
                let object_table = ObjectTableSingleton::get_singleton();

                let class = object_table.get_class(class_ref);

                let method = class.get_constant_pool_entry(pool_index);
                // TODO: Check if actually static
                match method {
                    PoolEntry::Method(Method::Native(native_method_index)) => {
                        let native_method = NativeMethodTable::get_table().get_method(native_method_index);
                        match native_method {
                            NativeMethod::Rust(method) => {
                                method(&[])?;
                            },
                        }
                    }
                    PoolEntry::Method(Method::Bytecode(_, method_index)) => {

                        let pc = self.stack.get_current_pc();
                        self.stack.set_current_pc(pc + 1);
                        self.stack.push_frame(self.stack.get_class_index(), method_index);
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
