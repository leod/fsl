use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use glow::HasContext;

use crate::{
    gl::{BufferError, BufferUsage, ProgramError},
    sl::program_def::ProgramDef,
};

use super::{
    disjoint_timer_query::DisjointTimerQuery, tracing::Tracing, Buffer, Caps, ContextError,
    DrawCallInfo, DrawParams, FrameTrace, Image, Program, Texture2d, TextureError,
};

pub(super) struct ContextShared {
    gl: glow::Context,
    caps: Caps,
    draw_params: Cell<DrawParams>,
    draw_fbo: glow::Framebuffer,
    default_framebuffer_size: Cell<[u32; 2]>,
    tracing: RefCell<Option<Tracing>>,
}

pub struct Context {
    shared: Rc<ContextShared>,
}

impl ContextShared {
    pub fn ref_eq(&self, other: &ContextShared) -> bool {
        std::ptr::eq(self as *const ContextShared, other as *const ContextShared)
    }

    pub fn gl(&self) -> &glow::Context {
        &self.gl
    }

    pub fn caps(&self) -> &Caps {
        &self.caps
    }

    pub(super) fn set_draw_params(&self, new: &DrawParams, framebuffer_size: [u32; 2]) {
        let gl = &self.gl;

        let current = self.draw_params.get();
        new.set_delta(gl, &current, framebuffer_size);
        self.draw_params.set(*new);
    }

    pub(super) fn draw_fbo(&self) -> glow::Framebuffer {
        self.draw_fbo
    }

    pub(super) fn default_framebuffer_size(&self) -> [u32; 2] {
        self.default_framebuffer_size.get()
    }
}

impl Context {
    pub fn new(gl: glow::Context) -> Result<Self, ContextError> {
        let caps = Caps::new(&gl);

        // All vertex bindings are made through a single vertex array object
        // that is bound at the start. The vertex array object binding must not
        // be changed during the lifetime of a context.
        let vao = unsafe { gl.create_vertex_array() }.map_err(ContextError::ObjectCreation)?;

        unsafe { gl.bind_vertex_array(Some(vao)) };

        // All framebuffer attachments are made with a single framebuffer object
        // that is created at the start.
        let draw_fbo = unsafe { gl.create_framebuffer() }.map_err(ContextError::ObjectCreation)?;

        let default_framebuffer_size = {
            let mut viewport = [0, 0, 0, 0];

            unsafe { gl.get_parameter_i32_slice(glow::VIEWPORT, &mut viewport) };

            [
                viewport[2].try_into().unwrap(),
                viewport[3].try_into().unwrap(),
            ]
        };

        let shared = Rc::new(ContextShared {
            gl,
            caps,
            draw_params: Cell::new(DrawParams::new()),
            draw_fbo,
            default_framebuffer_size: Cell::new(default_framebuffer_size),
            tracing: Default::default(),
        });

        Ok(Self { shared })
    }

    pub fn caps(&self) -> &Caps {
        &self.shared.caps
    }

    pub fn create_buffer(
        &self,
        data: &[u8],
        target: u32,
        usage: BufferUsage,
    ) -> Result<Buffer, BufferError> {
        Buffer::new(self.shared.clone(), data, target, usage)
    }

    pub fn create_texture_2d(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new(self.shared.clone(), image)
    }

    pub fn create_texture_2d_with_mipmap(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new_with_mipmap(self.shared.clone(), image)
    }

    pub fn create_program(
        &self,
        vertex_shader_name: String,
        fragment_shader_name: String,
        def: ProgramDef,
    ) -> Result<Program, ProgramError> {
        Program::new(
            self.shared.clone(),
            vertex_shader_name,
            fragment_shader_name,
            def,
        )
    }

    pub fn finish(&self) {
        unsafe { self.shared.gl.finish() };
    }

    pub fn default_framebuffer_size(&self) -> [u32; 2] {
        self.shared.default_framebuffer_size.get()
    }

    pub fn set_default_framebuffer_size(&self, size: [u32; 2]) {
        self.shared.default_framebuffer_size.set(size);
    }

    pub fn tracing_enable(&self) {
        *self.shared.tracing.borrow_mut() = Some(Default::default())
    }

    pub fn tracing_disable(&self) {
        *self.shared.tracing.borrow_mut() = None;
    }

    pub fn tracing_start_frame(&self) -> Option<FrameTrace> {
        let mut tracing = self.shared.tracing.borrow_mut();

        let result = tracing
            .as_mut()
            .and_then(move |tracing| tracing.start_frame());

        result
    }
}

pub(super) fn tracing_start_draw_call(
    ctx: &Rc<ContextShared>,
    info: impl Fn() -> DrawCallInfo,
) -> Option<Rc<DisjointTimerQuery>> {
    let mut tracing = ctx.tracing.borrow_mut();

    tracing.as_mut().and_then(|tracing| {
        tracing.start_draw_call(info(), || DisjointTimerQuery::new(ctx.clone()).ok())
    })
}
