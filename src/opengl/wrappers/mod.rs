use gl::types::GLuint;

pub mod Buffer;
pub mod Handler;
pub mod Shader;
pub mod Trash;
// pub mod Recorder;

#[derive(Debug, Copy, Clone)]
pub struct glBuffs {
    vertex: Option<GLuint>,
    normals: Option<GLuint>,
    indices: Option<GLuint>,
}
