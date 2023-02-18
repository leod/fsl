use crate::dag::{BuiltInType, SamplerType, Type};

/// Uniform input definition.
#[derive(Debug, Clone)]
pub struct UniformBlockDef {
    /// The name of the uniform block.
    pub block_name: String,

    /// The name of the single field within the uniform block.
    pub arg_name: String,

    /// The type of the uniform block.
    pub ty: Type,

    /// The location to which this uniform block is to be bound in the program.
    pub location: usize,
}

/// Sampler input definition.
#[derive(Debug, Clone)]
pub struct UniformSamplerDef {
    pub name: String,
    pub ty: SamplerType,
    pub texture_unit: usize,
}

/// Vertex attribute definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexAttributeDef {
    pub name: String,
    pub ty: BuiltInType,
    pub offset: usize,
}

/// The rate at which a particular vertex input advances.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VertexInputRate {
    Vertex,
    Instance,
}

/// Vertex input definition.
#[derive(Debug, Clone)]
pub struct VertexDef {
    pub input_rate: VertexInputRate,
    pub stride: usize,
    pub attributes: Vec<VertexAttributeDef>,
}

/// Type-erased definition of a program.
///
/// This is exposed for the purpose of inspecting the generated shader code. See
/// [`crate::util::compile_to_program_def`] for how to construct a type-erased
/// [`ProgramDef`] from typed shader definitions.
#[derive(Debug, Clone, Default)]
pub struct ProgramDef {
    /// Uniforms that the program needs.
    pub uniform_block_defs: Vec<UniformBlockDef>,

    /// Samplers that the program needs.
    pub uniform_sampler_defs: Vec<UniformSamplerDef>,

    /// Vertices that the program needs.
    pub vertex_defs: Vec<VertexDef>,

    /// Vertex shader source code.
    pub vertex_shader_source: String,

    /// Fragment shader source code.
    pub fragment_shader_source: String,
}
