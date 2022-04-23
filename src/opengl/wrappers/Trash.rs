use backtrace::Backtrace;
use std::sync::atomic::Ordering;

use crate::opengl::*;
use anyhow::Result;
use gl::{types::*, SAMPLES};

pub fn dump_errors() {
    unsafe {
        let mut err: GLenum = gl::GetError();

        // if err != gl::NO_ERROR {
        //     error!("{:?}", bt);
        // }

        loop {
            match err {
                gl::NO_ERROR => break,
                gl::INVALID_ENUM => {
                    error!("OpenGL!!! {}", "Invalid Enum");
                    break;
                }
                gl::INVALID_VALUE => {
                    error!("OpenGL!!! {}", "Invalid Value");
                    println!("{:?}", Backtrace::new());
                    panic!();
                }
                gl::INVALID_OPERATION => {
                    error!("OpenGL!!! {}", "Invalid Operation");
                    println!("{:?}", Backtrace::new());
                    panic!();
                }
                gl::INVALID_FRAMEBUFFER_OPERATION => {
                    error!("OpenGL!!! {}", "Invalid Framebuffer Operation");
                    println!("{:?}", Backtrace::new());
                    panic!();
                }
                gl::OUT_OF_MEMORY => {
                    error!("OpenGL!!! {}", "Out of Memory");
                    println!("{:?}", Backtrace::new());
                    panic!();
                }
                _ => {
                    panic!("Wtf? Wrong OpenGL error code???");
                }
            }
        }
    }
}

pub fn delete_shader(shader: GLuint) -> Result<()> {
    unsafe {
        gl::DeleteShader(shader);
    }
    SHADERS.fetch_sub(1, Ordering::SeqCst);

    #[cfg(debug_assertions)]
    dump_errors();
    Ok(())
}

pub fn delete_program(program: GLuint) -> Result<()> {
    unsafe {
        gl::DeleteProgram(program);
    }
    PROGRAMS.fetch_sub(1, Ordering::SeqCst);
    Ok(())
}

pub fn delete_vertex_array(vao: GLuint) -> Result<()> {
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
    }
    VERTEX_ARRAYS.fetch_sub(1, Ordering::SeqCst);
    Ok(())
}

pub fn delete_buffer(vbo: GLuint) -> Result<()> {
    unsafe {
        gl::DeleteBuffers(1, &vbo);
    }
    BUFFERS.fetch_sub(1, Ordering::SeqCst);
    Ok(())
}

pub fn delete_frame_buffer(fbo: GLuint) -> Result<()> {
    unsafe {
        gl::DeleteFramebuffers(1, &fbo);
    }
    FRAMEBUFFERS.fetch_sub(1, Ordering::SeqCst);
    Ok(())
}

pub fn delete_render_buffer(rbo: GLuint) -> Result<()> {
    unsafe {
        gl::DeleteRenderbuffers(1, &rbo);
    }
    RENDERBUFFERS.fetch_sub(1, Ordering::SeqCst);
    Ok(())
}

pub fn delete_samplers(sampler: GLuint) -> Result<()> {
    todo!()
    // unsafe { gl::DeleteSamplers(1, &sampler);}
    // SAMPLERS.fetch_sub(1, Ordering::SeqCst);
    // Ok(())
}

pub fn delete_texture(textures: *const GLuint) -> Result<()> {
    unsafe {
        gl::DeleteTextures(1, textures);
    }

    TEXTURES.fetch_sub(1, Ordering::SeqCst);

    Ok(())
}
