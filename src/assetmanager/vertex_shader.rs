use assets_manager::loader::Loader;
use assets_manager::Asset;
use assets_manager::BoxedError;
use assets_manager::Error;
use gl::types::GLuint;
use serde::Deserialize;
use std::borrow::Cow;
use std::sync::atomic::Ordering;

use crate::opengl::wrappers::Shader::compile_shader;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct VertexShader {
    pub shader: GLuint,
}

impl Asset for VertexShader {
    const EXTENSION: &'static str = "vert";

    // The serialization format (RON)
    type Loader = VertexShaderLoader;
}

pub struct VertexShaderLoader;
impl Loader<VertexShader> for VertexShaderLoader {
    fn load(content: Cow<[u8]>, id: &str) -> Result<VertexShader, BoxedError> {
        let src = std::str::from_utf8(&content).unwrap();

        // Tell the gl thread to hold the fuck up and wait for it to respond
        // if let Some("main") = std::thread::current().name() {
        // }else {
        //     GL_HALT.store(1, Ordering::Relaxed);
        //     while GL_HALT.load(Ordering::Relaxed) == 1 {std::thread::sleep_ms(500)};
        // }

        let vs =
            unsafe { compile_shader(&src.replace("\r", ""), gl::VERTEX_SHADER).unwrap() };

        // info!("Loaded \"{}\" vertex shader", id);

        // GL_HALT.store(0, Ordering::Relaxed);
        Ok(VertexShader { shader: vs })
    }
}
