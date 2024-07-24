pub mod ast;
pub mod token;
pub mod lexer;

lalrpop_util::lalrpop_mod!(pub grammar);


fn main() {
    println!("Hello, world!");
}
