use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::lang::{
    BinaryOp, BuiltInTy, Expr, Ident, Literal, LiteralExpr, ScalarTy, TernaryExpr, Ty,
};

use super::{binary, IntoPosh, Lift, Trace, TransparentValue, Type, Value};

pub trait ScalarType: Copy + Into<Literal> + IntoPosh<Posh = Scalar<Self>> {
    fn scalar_ty() -> ScalarTy;
}

pub trait NumericType: ScalarType {}

impl<T> Type for T
where
    T: ScalarType,
{
    fn ty() -> Ty {
        Ty::BuiltIn(BuiltInTy::Scalar(T::scalar_ty()))
    }
}

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    _phantom: PhantomData<T>,
    trace: Trace,
}

impl<T> Value for Scalar<T>
where
    T: ScalarType,
{
    type Type = T;

    fn from_ident(ident: Ident) -> Self {
        Self::from_trace(Trace::from_ident::<Self>(ident))
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }
}

impl<T> TransparentValue for Scalar<T>
where
    T: ScalarType,
{
    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Type as Type>::ty());

        Scalar {
            _phantom: PhantomData,
            trace,
        }
    }
}

impl<T> Scalar<T>
where
    T: ScalarType,
{
    pub fn new(x: T) -> Self {
        Self::from_expr(Expr::Literal(LiteralExpr { literal: x.into() }))
    }

    pub fn eq<V>(&self, right: impl IntoPosh<Posh = V>) -> Bool
    where
        V: Value<Type = T>,
    {
        binary(*self, BinaryOp::Eq, right)
    }
}

impl Bool {
    pub fn and(self, right: impl IntoPosh<Posh = Bool>) -> Bool {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl IntoPosh<Posh = Bool>) -> Bool {
        binary(self, BinaryOp::And, right)
    }

    pub fn ternary<V: TransparentValue>(
        self,
        true_value: impl IntoPosh<Posh = V>,
        false_value: impl IntoPosh<Posh = V>,
    ) -> V {
        let cond = Rc::new(self.expr());
        let true_expr = Rc::new(true_value.into_posh().expr());
        let false_expr = Rc::new(false_value.into_posh().expr());

        let expr = Expr::Ternary(TernaryExpr {
            cond,
            true_expr,
            false_expr,
        });

        V::from_expr(expr)
    }
}

macro_rules! impl_binary_op {
    ($fn:ident, $op:ident) => {
        impl<T, Rhs> $op<Rhs> for Scalar<T>
        where
            T: NumericType,
            Rhs: IntoPosh<Posh = Scalar<T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<Scalar<Self>> for f32 {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<Scalar<Self>> for i32 {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<Scalar<Self>> for u32 {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

impl_binary_op!(add, Add);
impl_binary_op!(sub, Sub);
impl_binary_op!(mul, Mul);
impl_binary_op!(div, Div);

macro_rules! impl_scalar {
    ($ty:ty, $name:ident) => {
        impl ScalarType for $ty {
            fn scalar_ty() -> ScalarTy {
                ScalarTy::$name
            }
        }

        impl Lift for $ty {
            type Posh = Scalar<$ty>;
        }

        impl IntoPosh for $ty {
            fn into_posh(self) -> Self::Posh {
                Scalar::new(self)
            }
        }

        pub type $name = Scalar<$ty>;
    };
}

impl_scalar!(f32, F32);
impl_scalar!(i32, I32);
impl_scalar!(u32, U32);
impl_scalar!(bool, Bool);

impl NumericType for f32 {}
impl NumericType for i32 {}
impl NumericType for u32 {}
