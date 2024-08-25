
mod vm;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = std::fs::read(filename).unwrap();

    let mut source = source.into_iter();
    let constant_pool = vm::ConstantPool::from_binary(&mut source);
    let slice = [source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap(), source.next().unwrap()];
    let block_count = u64::from_le_bytes(slice);
    let bytecode = vm::get_bytecode(&mut source);

    let mut jit = vm::Jit::new(constant_pool, bytecode);

    jit.run(block_count);
}
