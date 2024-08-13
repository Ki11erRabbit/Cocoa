use bytecode::Bytecode;

mod vm;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = std::fs::read_to_string(filename).unwrap();

    let mut source = source.into_bytes().into_iter();
    let constant_pool = vm::ConstantPool::from_binary(&mut source);
    let bytecode = vm::get_bytecode(&mut source);

    let mut jit = vm::Jit::new(constant_pool, bytecode);

    jit.run();
}
