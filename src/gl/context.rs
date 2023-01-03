use std::rc::Rc;

use crate::{Sl, ToPod, Vertex, VertexInterface};

use super::{
    untyped, BufferUsage, CreateBufferError, CreateVertexArrayError, Element, ElementBuffer,
    ElementSource, VertexArray, VertexBuffer,
};

pub struct Context {
    gl: Rc<glow::Context>,
    untyped: untyped::Context,
}

impl Context {
    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.gl
    }

    pub fn create_vertex_buffer<V: Vertex<Sl>>(
        &self,
        data: &[V::InGl],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<V>, CreateBufferError> {
        // TODO: This extra allocation for converting to `V::Pod` could be
        // eliminated if we see the need.
        let data: Vec<_> = data.iter().copied().map(ToPod::to_pod).collect();

        // TODO: We should also allow passing `V::Pod` directly.

        let untyped = self.untyped.create_buffer(&data, usage)?;

        Ok(VertexBuffer::from_untyped(untyped))
    }

    pub fn create_element_buffer<E: Element>(
        &self,
        data: &[E],
        usage: BufferUsage,
    ) -> Result<ElementBuffer<E>, CreateBufferError> {
        let untyped = self.untyped.create_buffer(data, usage)?;

        Ok(ElementBuffer::from_untyped(untyped))
    }

    pub fn create_vertex_array<V: VertexInterface<Sl>, E: ElementSource>(
        &self,
        vertex_buffers: V::InGl,
        element_source: E,
    ) -> Result<VertexArray<V, E>, CreateVertexArrayError> {
        VertexArray::new(self, vertex_buffers, element_source)
    }

    pub fn untyped(&self) -> &untyped::Context {
        &self.untyped
    }
}
