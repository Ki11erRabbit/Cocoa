pub mod bytecode;
pub mod module;



pub trait IntoBinary {
    fn into_binary(&self) -> Vec<u8>;
}


pub trait FromBinary: Sized {
    fn from_binary(source: &mut dyn Iterator<Item = u8>) -> Self;
}
