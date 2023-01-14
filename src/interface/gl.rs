use sealed::sealed;

use crate::{
    gl::{Sampler2dBinding, Texture2dBinding, UniformBufferBinding, VertexBuffer},
    program_def::{VertexAttributeDef, VertexInputRate},
    sl, Gl, Numeric, Sl,
};

use super::{
    FragmentInterface, FragmentInterfaceVisitor, Primitive, ResourceInterface, ToPod, Uniform,
    Vertex, VertexInterface, VertexInterfaceVisitor,
};

#[sealed]
impl super::Domain for Gl {
    type Scalar<T: Primitive> = T;
    type Vec2<T: Primitive> = mint::Vector2<T>;

    type Bool = bool;
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
}

// Uniform

impl Uniform<Gl> for bool {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Uniform<Gl> for f32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Uniform<Gl> for i32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Uniform<Gl> for u32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl<T: Primitive> Uniform<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = sl::Vec2<T>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

// Vertex

impl Vertex<Gl> for bool {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
        <Self::InSl as Vertex<Sl>>::attribute_defs(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Vertex<Gl> for f32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
        <Self::InSl as Vertex<Sl>>::attribute_defs(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Vertex<Gl> for i32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
        <Self::InSl as Vertex<Sl>>::attribute_defs(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Vertex<Gl> for u32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
        <Self::InSl as Vertex<Sl>>::attribute_defs(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl<T: Primitive> Vertex<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = sl::Vec2<T>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
        <Self::InSl as Vertex<Sl>>::attribute_defs(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

// VertexInterface

#[sealed]
impl super::VertexDomain for Gl {
    type Vertex<V: Vertex<Sl>> = VertexBuffer<V>;
}

impl<V: Vertex<Sl>> VertexInterface<Gl> for VertexBuffer<V> {
    type InGl = Self;
    type InSl = V::InSl;

    fn visit(&self, path: &str, visitor: &mut impl VertexInterfaceVisitor<Gl>) {
        visitor.accept(path, VertexInputRate::Vertex, self)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

#[sealed]
impl<V: Vertex<Sl>> super::VertexInterfaceField<Gl> for VertexBuffer<V> {
    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

// ResourceInterface

#[sealed]
impl super::ResourceDomain for Gl {
    type Sampler2d<T: Numeric> = Sampler2dBinding<T>;
    type Uniform<U: Uniform<Sl, InSl = U>> = UniformBufferBinding<U>;
    type Compose<R: ResourceInterface<Sl>> = R::InGl;
}

impl<T: Numeric> ResourceInterface<Gl> for Sampler2dBinding<T> {
    type InGl = Self;
    type InSl = sl::Sampler2d<T>;

    fn visit(&self, path: &str, visitor: &mut impl super::ResourceInterfaceVisitor<Gl>) {
        visitor.accept_sampler2d(path, self);
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl<U: Uniform<Sl, InSl = U>> ResourceInterface<Gl> for UniformBufferBinding<U> {
    type InGl = Self;
    type InSl = U;

    fn visit(&self, path: &str, visitor: &mut impl super::ResourceInterfaceVisitor<Gl>) {
        visitor.accept_uniform::<U::InSl>(path, self);
    }

    fn shader_input(_: &str) -> Self {
        todo!()
    }
}

// FragmentInterface

#[sealed]
impl super::FragmentDomain for Gl {
    type Attachment = Texture2dBinding;
}

impl FragmentInterface<Gl> for Texture2dBinding {
    type InGl = Self;
    type InSl = sl::Vec4<f32>;

    fn visit(&self, path: &str, visitor: &mut impl FragmentInterfaceVisitor<Gl>) {
        visitor.accept(path, self);
    }
}
