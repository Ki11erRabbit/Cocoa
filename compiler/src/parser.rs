use crate::lexer::SpannedToken;



pub struct Parser<'a> {
    tokens: Vec<SpannedToken<'a>>,
    index: usize,
}
