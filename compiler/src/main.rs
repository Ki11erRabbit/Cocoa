use backend::IntoBinary;
use parser::{ParseResult, ParserError};
use ariadne::{Label, Report, ReportKind, Source};


mod ast;
mod parser;
mod lexer;
mod backend;
mod typechecker;



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

    let mut typechecker = typechecker::TypeChecker::new();
    let statements = match typechecker.check_statements(statement) {
        Ok(statements) => statements,
        Err(()) => {
            for error in typechecker.errors.iter() {
                let report = Report::build(ReportKind::Error, filename, error.start)
                    .with_label(
                        Label::new((filename, error.start..error.end))
                            .with_message(error.message.clone())
                    );
                let mut report = if let Some(tip) = &error.tip {
                    report.with_note(tip.clone())
                } else {
                    report
                };
                for (start, end, message) in &error.additional {
                    report = report.with_label(
                        Label::new((filename, *start..*end))
                            .with_message(message.clone())
                    );
                }
                report.finish().print((filename, Source::from(source.clone()))).unwrap();
            }
            std::process::exit(1);
        }
    };

    let mut constant_pool = backend::ConstantPool::new();
    let mut backend = backend::StatementsCompiler::new();
    backend.start_compilation(&mut constant_pool, &statements);

    let mut output = constant_pool.into_binary();
    output.extend(backend.into_binary());

    let output_filename = format!("{}.bc", filename);
    std::fs::write(output_filename, output).unwrap();
}
