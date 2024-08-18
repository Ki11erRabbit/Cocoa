use either::Either;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SpannedType {
    pub type_: Type,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Type {
    Unit,
    Bool,
    Char,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Range(Box<Type>),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
            Self::Range(type_) => write!(f, "range<{}>", type_),
        }
    }
}

impl From<crate::ast::Type> for Type {
    fn from(type_: crate::ast::Type) -> Self {
        match type_ {
            crate::ast::Type::Unit => Self::Unit,
            crate::ast::Type::Bool => Self::Bool,
            crate::ast::Type::Char => Self::Char,
            crate::ast::Type::I8 => Self::I8,
            crate::ast::Type::I16 => Self::I16,
            crate::ast::Type::I32 => Self::I32,
            crate::ast::Type::I64 => Self::I64,
            crate::ast::Type::U8 => Self::U8,
            crate::ast::Type::U16 => Self::U16,
            crate::ast::Type::U32 => Self::U32,
            crate::ast::Type::U64 => Self::U64,
            crate::ast::Type::F32 => Self::F32,
            crate::ast::Type::F64 => Self::F64,
        }
    }
}


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
    LetStatement {
        binding: SpannedPattern,
        type_annotation: SpannedType,
        expression: SpannedExpression,
    },
    Assignment {
        binding: SpannedLhs,
        expression: SpannedExpression,
    },
    WhileStatement {
        condition: SpannedExpression,
        body: Vec<SpannedStatement>,
    },
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SpannedLhs {
    pub lhs: Lhs,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Lhs {
    Variable(String),
}


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SpannedPattern {
    pub pattern: Pattern,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Pattern {
    Identifier(String),
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
    Variable(String),
    Label {
        name: String,
        body: Either<Box<SpannedStatement>, Box<SpannedExpression>>,
    },
    LoopExpression {
        type_: Type,
        body: Vec<SpannedStatement>,
    },
    BreakExpression {
        type_: Option<Type>,
        label: Option<String>,
        expression: Option<Box<SpannedExpression>>,
    },
    ReturnExpression(Option<Box<SpannedExpression>>),
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

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "add (+)"),
            Self::Subtract => write!(f, "subtract (-)"),
            Self::Multiply => write!(f, "multiply (*)"),
            Self::Divide => write!(f, "divide (/)"),
            Self::Modulo => write!(f, "modulo (%)"),
            Self::Equal => write!(f, "equal (==)"),
            Self::NotEqual => write!(f, "not equal (!=)"),
            Self::LessThan => write!(f, "less than (<)"),
            Self::GreaterThan => write!(f, "greater than (>)"),
            Self::LessThanOrEqual => write!(f, "less than or equal (<=)"),
            Self::GreaterThanOrEqual => write!(f, "greater than or equal (>=)"),
            Self::And => write!(f, "and (&&)"),
            Self::Or => write!(f, "or (||)"),
            Self::BitwiseAnd => write!(f, "bitwise and (&)"),
            Self::BitwiseOr => write!(f, "bitwise or (|)"),
            Self::BitwiseXor => write!(f, "bitwise xor (^)"),
            Self::LeftShift => write!(f, "left shift (<<)"),
            Self::RightShift => write!(f, "right shift (>>)"),
            Self::ExclusiveRange => write!(f, "exclusive range (..)"),
            Self::InclusiveRange => write!(f, "inclusive range (..=)"),
        }
    }
}

impl From<crate::ast::BinaryOperator> for BinaryOperator {
    fn from(operator: crate::ast::BinaryOperator) -> Self {
        match operator {
            crate::ast::BinaryOperator::Add => Self::Add,
            crate::ast::BinaryOperator::Subtract => Self::Subtract,
            crate::ast::BinaryOperator::Multiply => Self::Multiply,
            crate::ast::BinaryOperator::Divide => Self::Divide,
            crate::ast::BinaryOperator::Modulo => Self::Modulo,
            crate::ast::BinaryOperator::Equal => Self::Equal,
            crate::ast::BinaryOperator::NotEqual => Self::NotEqual,
            crate::ast::BinaryOperator::LessThan => Self::LessThan,
            crate::ast::BinaryOperator::GreaterThan => Self::GreaterThan,
            crate::ast::BinaryOperator::LessThanOrEqual => Self::LessThanOrEqual,
            crate::ast::BinaryOperator::GreaterThanOrEqual => Self::GreaterThanOrEqual,
            crate::ast::BinaryOperator::And => Self::And,
            crate::ast::BinaryOperator::Or => Self::Or,
            crate::ast::BinaryOperator::BitwiseAnd => Self::BitwiseAnd,
            crate::ast::BinaryOperator::BitwiseOr => Self::BitwiseOr,
            crate::ast::BinaryOperator::BitwiseXor => Self::BitwiseXor,
            crate::ast::BinaryOperator::LeftShift => Self::LeftShift,
            crate::ast::BinaryOperator::RightShift => Self::RightShift,
            crate::ast::BinaryOperator::ExclusiveRange => Self::ExclusiveRange,
            crate::ast::BinaryOperator::InclusiveRange => Self::InclusiveRange,
        }
    }
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
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Bool(bool),
}
