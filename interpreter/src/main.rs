use definitions::{FromBinary, Module};


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
    let module = Module::from_binary(&mut source);

    let mut jit = vm::Jit::new(&module);

    jit.run();
}
