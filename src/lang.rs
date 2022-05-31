pub mod show;

pub use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScalarType {
    Bool,
    U32,
    F32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Scalar(ScalarType),
    Vec3(ScalarType),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    pub name: String,
    pub uuid: Uuid,
}

impl Ident {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uuid: Uuid::new_v4(),
        }
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        format!("{}_{}", self.name, &self.uuid.simple().to_string()[0..8])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var {
    pub ident: Ident,
    pub ty: Type,
    pub init: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Func {
    BuiltIn(FuncBuiltIn),
    UserDefined(FuncUserDefined),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncBuiltIn {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncUserDefined {
    pub ident: Ident,
    pub params: Vec<Var>,
    pub result: Box<Expr>,
}

impl Func {
    pub fn ty(&self) -> Type {
        use Func::*;

        match self {
            BuiltIn(FuncBuiltIn { ty, .. }) => ty.clone(),
            UserDefined(FuncUserDefined { result, .. }) => result.ty(),
        }
    }

    pub fn name(&self) -> &str {
        use Func::*;

        match self {
            BuiltIn(FuncBuiltIn { name, .. }) => name,
            UserDefined(FuncUserDefined { ident, .. }) => &ident.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lit {
    pub value: String,
    pub ty: Type,
}

impl From<bool> for Lit {
    fn from(x: bool) -> Self {
        Self {
            value: x.to_string(),
            ty: Type::Scalar(ScalarType::Bool),
        }
    }
}

impl From<u32> for Lit {
    fn from(x: u32) -> Self {
        Self {
            value: x.to_string(),
            ty: Type::Scalar(ScalarType::U32),
        }
    }
}

impl From<f32> for Lit {
    fn from(x: f32) -> Self {
        Self {
            value: x.to_string(),
            ty: Type::Scalar(ScalarType::F32),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    Binary(ExprBinary),
    Ternary(ExprTernary),
    Var(ExprVar),
    Call(ExprCall),
    Lit(ExprLit),
    Field(ExprField),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprTernary {
    pub cond: Box<Expr>,
    pub true_expr: Box<Expr>,
    pub false_expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprVar {
    pub var: Var,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprCall {
    pub func: Func,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprLit {
    pub lit: Lit,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprField {
    pub base: Box<Expr>,
    pub member: String,
    pub ty: Type,
}

impl Expr {
    pub fn ty(&self) -> Type {
        use Expr::*;

        match self {
            Binary(expr) => expr.ty.clone(),
            Ternary(expr) => {
                assert!(expr.true_expr.ty() == expr.false_expr.ty());
                expr.true_expr.ty()
            }
            Var(expr) => expr.var.ty.clone(),
            Call(expr) => expr.func.ty(),
            Lit(expr) => expr.lit.ty.clone(),
            Field(expr) => expr.ty.clone(),
        }
    }
}
