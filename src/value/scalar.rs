use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{
    expr_reg::ExprId,
    lang::{BinOp, Expr, ExprLit, Lit, Type},
};

use super::{binary, Value, ValueType};

pub trait ScalarValueType: Copy + Clone + ValueType + Into<Lit> {}

pub trait NumericValueType: ScalarValueType {}

impl<T> Value for T
where
    T: ScalarValueType,
{
    type Type = T;

    fn from_expr_id(_: ExprId) -> Self {
        unimplemented!();
    }

    fn expr_id(&self) -> ExprId {
        Scalar::new(*self).expr_id
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    _phantom: PhantomData<T>,
    expr_id: ExprId,
}

impl<T> Value for Scalar<T>
where
    T: ScalarValueType,
{
    type Type = T;

    fn from_expr_id(expr_id: ExprId) -> Self {
        Scalar {
            _phantom: PhantomData,
            expr_id,
        }
    }

    fn expr_id(&self) -> ExprId {
        self.expr_id
    }
}

impl<T> Scalar<T>
where
    T: ScalarValueType,
{
    pub fn new(x: T) -> Self {
        Self::from_expr(Expr::Lit(ExprLit { lit: x.into() }))
    }

    pub fn eq<V>(&self, right: V) -> Bool
    where
        V: Value<Type = T>,
    {
        binary(*self, BinOp::Eq, right)
    }
}

impl<T> From<T> for Scalar<T>
where
    T: ScalarValueType,
{
    fn from(x: T) -> Self {
        Self::new(x)
    }
}

impl<T, Rhs> Add<Rhs> for Scalar<T>
where
    T: NumericValueType,
    Rhs: Into<Scalar<T>>,
{
    type Output = Scalar<T>;

    fn add(self, right: Rhs) -> Self::Output {
        binary(self, BinOp::Add, right.into())
    }
}

impl<T, Rhs> Mul<Rhs> for Scalar<T>
where
    T: NumericValueType,
    Rhs: Into<Scalar<T>>,
{
    type Output = Scalar<T>;

    fn mul(self, right: Rhs) -> Self::Output {
        binary(self, BinOp::Mul, right.into())
    }
}

impl ValueType for bool {
    type Value = Scalar<bool>;

    fn ty() -> Type {
        Type::U32
    }
}

impl ValueType for u32 {
    type Value = Scalar<u32>;

    fn ty() -> Type {
        Type::U32
    }
}

impl ValueType for f32 {
    type Value = Scalar<f32>;

    fn ty() -> Type {
        Type::F32
    }
}

impl ScalarValueType for bool {}
impl ScalarValueType for u32 {}
impl ScalarValueType for f32 {}

impl NumericValueType for u32 {}
impl NumericValueType for f32 {}

pub type Bool = <bool as ValueType>::Value;
pub type U32 = <u32 as ValueType>::Value;
pub type F32 = <f32 as ValueType>::Value;
