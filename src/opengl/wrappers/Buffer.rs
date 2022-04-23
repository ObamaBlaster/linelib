use crate::opengl::{BUFFERS, FRAMEBUFFERS, RENDERBUFFERS, TEXTURES, VERTEX_ARRAYS};
use anyhow::Result;
use gl::types::*;
use std::{mem, ptr, sync::atomic::Ordering};

use super::Trash;

// TODO: https://www.khronos.org/opengl/wiki/Vertex_Rendering#Instancing
pub fn vertex_buffer_alloc(size: u32) -> GLuint {
    // let mut vbo = 0;
    // unsafe {
    //     gl::GenBuffers(1, &mut vbo);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //             size,
    //             ptr::null_mut(),
    //         gl::DYNAMIC_DRAW,
    //     );
    // }
    // vbo
    0
}

pub fn vertex_array(size: GLsizei) -> GLuint {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(size, &mut vao);
        #[cfg(debug_assertions)]
        Trash::dump_errors();

        VERTEX_ARRAYS.fetch_add(1, Ordering::SeqCst);
        gl::BindVertexArray(vao);
        #[cfg(debug_assertions)]
        Trash::dump_errors();
    }

    vao
}

pub fn gl_buffer<T>(buf: Vec<T>, binding: GLenum) -> GLuint {
    let mut bo = 0;
    unsafe {
        gl::GenBuffers(1, &mut bo);
        #[cfg(debug_assertions)]
        Trash::dump_errors();
        BUFFERS.fetch_add(1, Ordering::SeqCst);
        gl::BindBuffer(binding, bo);
        #[cfg(debug_assertions)]
        Trash::dump_errors();
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (buf.len() * mem::size_of::<T>()) as GLsizeiptr,
            mem::transmute(buf.as_ptr()),
            gl::STATIC_DRAW,
        );
        #[cfg(debug_assertions)]
        Trash::dump_errors();
    }

    bo
}

pub fn vertex_buffer(verts: Vec<f32>) -> GLuint {
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        #[cfg(debug_assertions)]
        Trash::dump_errors();
        BUFFERS.fetch_add(1, Ordering::SeqCst);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        #[cfg(debug_assertions)]
        Trash::dump_errors();
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (verts.len() * mem::size_of::<f32>()) as isize,
            mem::transmute(verts.as_ptr()),
            gl::STATIC_DRAW,
        );
        #[cfg(debug_assertions)]
        Trash::dump_errors();
    }
    vbo
}

pub fn twodee_vertex_array_object(verts: Vec<f32>, indices: Vec<u32>) -> (GLuint, GLuint, GLuint) {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }
    let vbo = vertex_buffer(verts);
    let ebo = index_buffer(indices);

    unsafe {
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (2 * mem::size_of::<f32>()) as i32,
            ptr::null(),
        )
    };
    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    (vao, vbo, ebo)
}

pub fn threedee_vertex_array_object(
    verts: Vec<f32>,
    indices: Vec<GLuint>,
) -> (GLuint, GLuint, GLuint) {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }
    let vbo = vertex_buffer(verts);
    let ebo = index_buffer(indices);

    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (2 * mem::size_of::<f32>()) as i32,
            ptr::null(),
        )
    };
    unsafe {
        gl::EnableVertexAttribArray(0);
    }
    (vao, ebo, ebo)
}

pub fn index_buffer(verts: Vec<GLuint>) -> GLuint {
    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        #[cfg(debug_assertions)]
        Trash::dump_errors();

        BUFFERS.fetch_add(1, Ordering::SeqCst);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        #[cfg(debug_assertions)]
        Trash::dump_errors();
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (mem::size_of::<i32>() * verts.len()) as isize,
            mem::transmute(verts.as_ptr()),
            gl::STATIC_DRAW,
        );
        #[cfg(debug_assertions)]
        Trash::dump_errors();
    }
    ebo
}

pub fn frame_buffer(WIDTH: u32, HEIGHT: u32) -> (GLuint, GLuint, GLuint) {
    let mut fbo = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut fbo);

        FRAMEBUFFERS.fetch_add(1, Ordering::SeqCst);
        #[cfg(debug_assertions)]
        Trash::dump_errors();
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
        #[cfg(debug_assertions)]
        Trash::dump_errors();
    }
    let mut texture = texture(WIDTH, HEIGHT);

    let mut rbo = 0;

    unsafe {
        // Adding framebuffer attatchments
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture,
            0,
        );

        // // Initialie the rnderbuffer object for storing the stencil and edepth
        gl::GenRenderbuffers(1, &mut rbo);
        RENDERBUFFERS.fetch_add(1, Ordering::SeqCst);

        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            WIDTH as GLint,
            HEIGHT as GLint,
        );
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            error!("Framebuffer failed to initialize :(");
            panic!();
        }
    }

    return (fbo, texture, rbo);
}

pub fn texture(WIDTH: u32, HEIGHT: u32) -> GLuint {
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        #[cfg(debug_assertions)]
        Trash::dump_errors();

        gl::BindTexture(gl::TEXTURE_2D, texture);
        #[cfg(debug_assertions)]
        Trash::dump_errors();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            WIDTH as i32,
            HEIGHT as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            ptr::null(),
        );

        #[cfg(debug_assertions)]
        Trash::dump_errors();

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        #[cfg(debug_assertions)]
        Trash::dump_errors();

        TEXTURES.fetch_add(1, Ordering::SeqCst);
    }

    return texture;
}
