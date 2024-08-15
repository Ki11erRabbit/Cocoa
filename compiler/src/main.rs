use backend::IntoBinary;
use parser::{ParseResult, ParserError};
use ariadne::{Label, Report, ReportKind, Source};


mod ast;
mod parser;
mod lexer;
mod backend;



fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = std::fs::read_to_string(filename).unwrap();

    let mut parser = parser::Parser::new(&source);

    let statement = match parser.parse_block_body() {
        Result::Ok(statement) => statement,
        Result::Err(ParserError::Error { message, start, end }) => {
            Report::build(ReportKind::Error, filename, start)
                .with_label(
                    Label::new((filename, start..end))
                        .with_message(message)
                )
                .finish()
                .print((filename, Source::from(source)))
                .unwrap();

            std::process::exit(1);
        }
        Result::Err(ParserError::EOF) => {
            eprintln!("Unexpected end of file");
            std::process::exit(1);
        }
    };

    let mut constant_pool = backend::ConstantPool::new();
    let mut backend = backend::StatementsCompiler::new();
    backend.compile_statements(&mut constant_pool, &statement);

    let mut output = constant_pool.into_binary();
    output.extend(backend.into_binary());

    let output_filename = format!("{}.bc", filename);
    std::fs::write(output_filename, output).unwrap();
}
