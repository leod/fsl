use posh::{
    sl::{self, Value},
    Block, BlockDom, Sl, Uniform, UniformDom, Vertex, VertexDom,
};

#[derive(Clone, Copy, Value)]
struct Foo {
    x: sl::Vec3,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct MyUniform1<D: BlockDom = Sl> {
    x: D::Vec2,
    y: D::IVec2,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct MyUniform2<D: BlockDom = Sl> {
    x: D::Vec2,
    y: MyUniform1<D>,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct MyVertex<D: BlockDom = Sl> {
    x: D::F32,
    y: D::Vec2,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct MyNestedVertex<D: BlockDom = Sl> {
    x: D::F32,
    zzz: MyUniform1<D>,
    y: D::Vec2,
}

#[derive(Copy, Clone, Vertex)]
struct MyVertexIface<D: VertexDom = Sl> {
    vertex: D::Block<MyVertex>,
    instance: D::Block<MyNestedVertex>,
}

#[derive(Uniform)]
struct MyUniformIface<D: UniformDom = Sl> {
    uniform: D::Block<MyUniform1>,
}

#[derive(Uniform)]
struct MyUniformIface2<D: UniformDom = Sl> {
    uniformxy: D::Block<MyUniform1>,
    bla: MyUniformIface<D>,
    zzz: D::Block<MyUniform1>,
}

#[derive(Uniform)]
struct GenericUniformIface<U, D: UniformDom = Sl>
where
    U: Uniform<Sl>,
{
    uniformxy: D::Block<MyUniform1>,
    x: D::Uniform<U>,
}

fn main() {}
