use crate::opengl::*;

// Performs a reference count check
pub fn check_gl() {
    debug!("Checking OpenGL context state");
    debug!("IF ANYTHING IS > 0 THERE IS A MEMLEAK");

    warn!("Vertex Arrays Remaning: {:?}", VERTEX_ARRAYS);
    warn!("Buffers Remaning: {:?}", BUFFERS);
    warn!("Framebuffers Remaning: {:?}", FRAMEBUFFERS);
    warn!("Renderbuffers Remaning: {:?}", RENDERBUFFERS);
    warn!("Shaders Remaning: {:?}", SHADERS);
    warn!("Programs Remaning: {:?}", PROGRAMS);

    warn!("Textures Remaning: {:?}", TEXTURES);
    warn!("Samplers Remaning: {:?}", SAMPLERS);
}
