use sealed::sealed;

use crate::{expose::NumType, lang::Ty, Scalar, Value, Vec2, Vec3, Vec4};

use super::fields::{add_prefix, Fields};

/// A representative that can be stored in a [`Vertex`].
#[sealed]
pub trait VertexField: Value {
    #[doc(hidden)]
    fn must_impl() {}
}

#[sealed]
impl<T: NumType> VertexField for Scalar<T> {}

#[sealed]
impl<T: NumType> VertexField for Vec2<T> {}

#[sealed]
impl<T: NumType> VertexField for Vec3<T> {}

#[sealed]
impl<T: NumType> VertexField for Vec4<T> {}

// TODO: VertexFieldValue for f32 matrix types

/// A representative of a vertex.
pub trait Vertex: Value + Fields {}

/// A representative of vertex stage input.
pub trait Attributes: Value + Fields {}

impl<V: Vertex> Attributes for V {}

impl<V0: Vertex, V1: Vertex> Fields for (V0, V1) {
    fn fields(prefix: &str) -> Vec<(String, Ty)> {
        V0::fields(&add_prefix(prefix, "x0"))
            .into_iter()
            .chain(V1::fields(&add_prefix(prefix, "x1")))
            .collect()
    }

    fn stage_input(prefix: &str) -> Self {
        (
            V0::stage_input(&add_prefix(prefix, "x0")),
            V1::stage_input(&add_prefix(prefix, "x1")),
        )
    }
}

impl<V1: Vertex, V2: Vertex> Attributes for (V1, V2) {}

/// A representative of vertex stage output and fragment stage input.
pub trait Interpolants: Value {}

/// A representative that can be stored in [`Interpolants`].
#[sealed]
pub trait InterpolantsField {
    #[doc(hidden)]
    fn must_impl() {}
}

#[sealed]
impl<T: NumType> InterpolantsField for Scalar<T> {}

#[sealed]
impl<T: NumType> InterpolantsField for Vec2<T> {}

#[sealed]
impl<T: NumType> InterpolantsField for Vec3<T> {}

#[sealed]
impl<T: NumType> InterpolantsField for Vec4<T> {}

// TODO: InterpolantsField for f32 matrix types
// TODO: InterpolantsField for arrays

#[sealed]
impl<V: Interpolants> InterpolantsField for V {}

/// A representative of fragment stage output.
pub trait Fragment: Value {}

/// A representative that can be stored in a [`Fragment`].
#[sealed]
pub trait FragmentField: Value {
    #[doc(hidden)]
    fn must_impl() {}
}

#[sealed]
impl<T: NumType> FragmentField for Scalar<T> {}

#[sealed]
impl<T: NumType> FragmentField for Vec2<T> {}

#[sealed]
impl<T: NumType> FragmentField for Vec3<T> {}

#[sealed]
impl<T: NumType> FragmentField for Vec4<T> {}
