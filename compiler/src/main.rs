use backend::IntoBinary;
use parser::ParseResult;

mod ast;
mod parser;
mod lexer;
mod backend;



fn main() -> ParseResult<'static, ()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = std::fs::read_to_string(filename).unwrap();

    let mut parser = parser::Parser::new(&source);

    let statement = parser.parse_block_body()?;

    let mut constant_pool = backend::ConstantPool::new();
    let mut backend = backend::StatementsCompiler::new();
    backend.compile_statements(&mut constant_pool, &statement);

    let mut output = constant_pool.into_binary();
    output.extend(backend.into_binary());

    let output_filename = format!("{}.bc", filename);
    std::fs::write(output_filename, output).unwrap();
    Ok(())
}
