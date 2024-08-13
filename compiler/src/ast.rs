




#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SpannedStatement {
    pub statement: Statement,
    pub start: usize,
    pub end: usize,
}



#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Statement {
    Expression(SpannedExpression),
    HangingExpression(SpannedExpression),
}


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SpannedExpression {
    pub expression: Expression,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expression {
    BinaryExpression {
        left: Box<SpannedExpression>,
        operator: BinaryOperator,
        right: Box<SpannedExpression>,
    },
    PrefixExpression {
        operator: PrefixOperator,
        right: Box<SpannedExpression>,
    },
    PostfixExpression {
        left: Box<SpannedExpression>,
        operator: PostfixOperator,
    },
    Literal(Literal),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    ExclusiveRange,
    InclusiveRange,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PrefixOperator {
    Not,
    Negate,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PostfixOperator {
    Try,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Int(i128),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Bool(bool),
}
