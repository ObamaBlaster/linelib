use gl::types::GLuint;
use assets_manager::loader::Loader;
use assets_manager::Asset;
use assets_manager::BoxedError;
use std::borrow::Cow;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use crate::opengl::wrappers::Shader::compile_shader;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct FragmentShader {
    pub shader: GLuint,
}

impl Asset for FragmentShader {
    const EXTENSION: &'static str = "frag";

    // The serialization format (RON)
    type Loader = FragmentShaderLoader;
}

pub struct FragmentShaderLoader;
impl Loader<FragmentShader> for FragmentShaderLoader {
    fn load(content: Cow<[u8]>, id: &str) -> Result<FragmentShader, BoxedError> {
        let src = std::str::from_utf8(&content).unwrap();

        // Tell the gl thread to hold the fuck up and wait for it to respond
        // if let Some("main") = std::thread::current().name() {
        // }
        // else{
        //     GL_HALT.store(1, Ordering::Relaxed);
        //     while GL_HALT.load(Ordering::Relaxed) == 1 {std::thread::sleep_ms(500)};
        // }

        let fs =
            unsafe { compile_shader(&src.replace("\r", ""), gl::FRAGMENT_SHADER).unwrap() };

        // info!("Loaded \"{}\" fragment shader", id);
        // GL_HALT.store(0, Ordering::Relaxed);
        Ok(FragmentShader { shader: fs })
    }
}