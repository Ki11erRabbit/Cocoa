use either::Either;


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct File {
    package_declaration: PackageDeclaration,
    import_declarations: Vec<ImportDeclaration>,
    primary_class: ClassDeclaration,
}

impl File {
    pub fn new(
        package_declaration: PackageDeclaration,
        import_declarations: Vec<ImportDeclaration>,
        primary_class: ClassDeclaration,
    ) -> Self {
        Self {
            package_declaration,
            import_declarations,
            primary_class,
        }
    }
}

pub type PackagePath = Vec<String>;
pub type ImportItem = String;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PackageDeclaration {
    path: PackagePath,
}

impl PackageDeclaration {
    pub fn new(path: PackagePath) -> Self {
        Self { path }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ImportDeclaration {
    path: ImportPath,
}

impl ImportDeclaration {
    pub fn new(path: ImportPath) -> Self {
        Self { path }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ImportPath {
    path: PackagePath,
    item: ImportItem,
}

impl ImportPath {
    pub fn new(path: PackagePath, item: ImportItem) -> Self {
        Self { path, item }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SuperClass {
    path: ImportPath,
    type_arguments: Vec<TypeParameter>,
}

impl SuperClass {
    pub fn new(path: ImportPath, type_arguments: Vec<TypeParameter>) -> Self {
        Self {
            path,
            type_arguments,
        }
    }
}

pub enum Declaration {
    Field(Field),
    Method(MethodDeclaration),
    Class(ClassDeclaration),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ClassDeclaration {
    visibility: Visibility,
    name: String,
    class_type: ClassType,
    type_parameters: Vec<TypeParameter>,
    super_class: Option<SuperClass>,
    interfaces: Vec<SuperClass>,
    decs: Vec<Declaration>,
}

impl ClassDeclaration {
    pub fn new(
        visibility: Visibility,
        name: String,
        class_type: ClassType,
        type_parameters: Vec<TypeParameter>,
        super_class: Option<SuperClass>,
        interfaces: Vec<SuperClass>,
        decs: Vec<Declaration>,
    ) -> Self {
        Self {
            visibility,
            name,
            class_type,
            type_parameters,
            super_class,
            interfaces,
            decs,
        }
    }
}

pub struct ClassDeclarationBuilder {
    visibility: Visibility,
    name: String,
    class_type: ClassType,
    type_parameters: Vec<TypeParameter>,
    super_class: Option<SuperClass>,
    interfaces: Vec<SuperClass>,
    decs: Vec<Declaration>,
}

impl ClassDeclarationBuilder {
    pub fn new() -> Self {
        Self {
            visibility: Visibility::Public,
            name: String::new(),
            class_type: ClassType::Class,
            type_parameters: Vec::new(),
            super_class: None,
            interfaces: Vec::new(),
            decs: Vec::new(),
        }
    }

    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn class_type(mut self, class_type: ClassType) -> Self {
        self.class_type = class_type;
        self
    }

    pub fn type_parameters(mut self, type_parameters: Vec<TypeParameter>) -> Self {
        self.type_parameters = type_parameters;
        self
    }

    pub fn super_class(mut self, super_class: Option<SuperClass>) -> Self {
        self.super_class = super_class;
        self
    }

    pub fn decs(mut self, decs: Vec<Declaration>) -> Self {
        self.decs = decs;
        self

    pub fn sub_classes(mut self, sub_classes: Vec<ClassDeclaration>) -> Self {
        self.sub_classes = sub_classes;
        self
    }

    pub fn interfaces(mut self, interfaces: Vec<SuperClass>) -> Self {
        self.interfaces = interfaces;
        self
    }

    pub fn build(self) -> ClassDeclaration {
        ClassDeclaration {
            visibility: self.visibility,
            name: self.name,
            class_type: self.class_type,
            type_parameters: self.type_parameters,
            super_class: self.super_class,
            interfaces: self.interfaces,
            decs: self.decs,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ClassType {
    Class,
    AbstractClass,
    Interface,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Visibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TypeParameter {
    name: String,
    bounds: Vec<Type>,
}

impl TypeParameter {
    pub fn new(name: String, bounds: Vec<Type>) -> Self {
        Self { name, bounds }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Field {
    visibility: Visibility,
    name: String,
    field_type: Type,
}

impl Field {
    pub fn new(visibility: Visibility, name: String, field_type: Type) -> Self {
        Self {
            visibility,
            name,
            field_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Type {
    Primitive(PrimitiveType),
    ClassType(ImportPath),
    Array(Box<Type>),
    TypeArguments(Box<Type>, Vec<Type>),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum PrimitiveType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Char,
    Unit,
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MethodDeclaration {
    visibility: Visibility,
    is_static: bool,
    name: String,
    type_parameters: Vec<TypeParameter>,
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Either<Option<Statement>, usize>,
}

impl MethodDeclaration {
    pub fn new(
        visibility: Visibility,
        is_static: bool,
        name: String,
        type_parameters: Vec<TypeParameter>,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Either<Option<Statement>, usize>,
    ) -> Self {
        Self {
            visibility,
            is_static,
            type_parameters,
            name,
            parameters,
            return_type,
            body,
        }
    }
}

pub struct MethodDeclarationBuilder {
    visibility: Visibility,
    is_static: bool,
    name: String,
    type_parameters: Vec<TypeParameter>,
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Either<Option<Statement>, usize>,
}

impl MethodDeclarationBuilder {
    pub fn new() -> Self {
        Self {
            visibility: Visibility::Public,
            is_static: false,
            name: String::new(),
            type_parameters: Vec::new(),
            parameters: Vec::new(),
            return_type: Type::Primitive(PrimitiveType::Unit),
            body: Either::Left(None),
        }
    }

    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn is_static(mut self, is_static: bool) -> Self {
        self.is_static = is_static;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn type_parameters(mut self, type_parameters: Vec<TypeParameter>) -> Self {
        self.type_parameters = type_parameters;
        self
    }

    pub fn parameters(mut self, parameters: Vec<Parameter>) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn return_type(mut self, return_type: Type) -> Self {
        self.return_type = return_type;
        self
    }

    pub fn body(mut self, body: Either<Option<Statement>, usize>) -> Self {
        self.body = body;
        self
    }

    pub fn build(self) -> MethodDeclaration {
        MethodDeclaration {
            visibility: self.visibility,
            is_static: self.is_static,
            name: self.name,
            type_parameters: self.type_parameters,
            parameters: self.parameters,
            return_type: self.return_type,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Parameter {
    name: String,
    parameter_type: Type,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Statement {
    Block(Vec<Statement>),
    While(WhileStatement),
    For(ForStatement),
    Return(Option<Expression>),
    Break,
    Continue,
    Expression(Expression),
    Let(String, Option<Type>, Expression),
    HangingExpression(Expression),
    If(IfExpression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WhileStatement {
    condition: Expression,
    body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ForStatement {
    variable: String,
    iterable: Expression,
    body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    FieldAccess(Box<Expression>, String),
    Call(Box<Expression>, Vec<Expression>),
    StaticAccess(ImportPath, String),
    New(ImportPath),
    ArrayAccess(Box<Expression>, Box<Expression>),
    ArrayCreation(Vec<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Cast(Type, Box<Expression>),
    Paren(Box<Expression>),
    If(IfExpression),
    This,
    Super,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
    Closure(Vec<Parameter>, Either<Vec<Statement>, Box<Expression>>),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    RightShift,
    LeftShift,
    BitAnd,
    BitOr,
    BitXor,
    InstanceOf,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    ExclusiveRange,
    InclusiveRange,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IfExpression {
    condition: Box<Expression>,
    then: Box<Statement>,
    else_: Option<Either<Box<Statement>, Box<IfExpression>>>,
}
