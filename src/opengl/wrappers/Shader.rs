use crate::nalgebra::Matrix4;
use crate::opengl::{PROGRAMS, SHADERS};
use crate::{
    opengl::wrappers::Trash::{self, delete_shader},
    read_to_string,
};
use anyhow::{Error, Result};
use gl::types::*;
use nalgebra_glm as glm;
use nalgebra::{Matrix, Matrix2};
use std::*;
use std::{ffi::*, sync::atomic::Ordering};

// Because I'm targeting OpenGL 3.0 devices
// we can't use the nicer (21st century) OpenGL debugging

pub unsafe fn compile_shader(src: &str, ty: GLenum) -> Result<GLuint> {
    #[cfg(feature = "shader_log")]
    {
        log::debug!("{}", src);
        log::debug!("============================");
    }

    unsafe {
        Trash::dump_errors();

        let shader: GLuint = gl::CreateShader(ty);
        SHADERS.fetch_add(1, Ordering::SeqCst);

        Trash::dump_errors();

        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        Trash::dump_errors();

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        Trash::dump_errors();

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buf = vec![0 as GLchar; len as usize];
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr());

            if ty == gl::VERTEX_SHADER {
                error!("Failed to compile vertex shader :(");
            } else if ty == gl::FRAGMENT_SHADER {
                error!("Failed to compile fragment shader :(");
            } else {
                error!("Failed to compile unkown shader :?");
            }

            return Err(Error::msg(
                CStr::from_ptr(buf.as_mut_ptr())
                    .to_str()
                    .unwrap()
                    .to_string(),
            ));
        }
        Ok(shader)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Uniform {
    Matrix4fv(GLsizei, GLboolean, [[GLfloat; 4]; 4]),
    Vector3fv(GLsizei, [GLfloat; 3]),
}

pub unsafe fn shader_args(program: GLuint, args: &[(&str, Uniform)]) {
    //#[cfg(feature = "shader_log")]
    // {
    //     log::debug!("glProgram : {}", program);
    //     log::debug!("Args: {:?}", args);
    //     log::debug!("============================");
    // }
    gl::UseProgram(program);
    #[cfg(debug_assertions)]
    Trash::dump_errors();

    for (name, args) in args.into_iter() {
        let loc = gl::GetUniformLocation(program, CString::new(*name).unwrap().into_raw());

        #[cfg(debug_assertions)]
        Trash::dump_errors();

        apply_args(loc, *args);
        // gl::UniformMatrix4fv(loc, 1, gl::FALSE, *data);

        #[cfg(debug_assertions)]
        Trash::dump_errors();
    }
}

// Use this as a reference
// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform.xhtml
unsafe fn apply_args(loc: GLint, uniform: Uniform) {
    match uniform {
        Uniform::Matrix4fv(size, trans, data) => {
            gl::UniformMatrix4fv(loc, size, gl::FALSE, std::mem::transmute(data.as_ptr()));
        }
        Uniform::Vector3fv(size, data) => {
            gl::Uniform3fv(loc, size, std::mem::transmute(data.as_ptr()));
        }
    }
}

pub unsafe fn link_program(vs: GLuint, fs: GLuint) -> Result<GLuint> {
    #[cfg(feature = "shader_log")]
    {
        log::debug!("Linking glPrograms : {} {}", vs, fs);
        log::debug!("============================");
    }

    let program = gl::CreateProgram();
    PROGRAMS.fetch_add(1, Ordering::SeqCst);
    #[cfg(debug_assertions)]
    Trash::dump_errors();

    gl::AttachShader(program, vs);
    #[cfg(debug_assertions)]
    Trash::dump_errors();

    gl::AttachShader(program, fs);

    #[cfg(debug_assertions)]
    Trash::dump_errors();

    gl::LinkProgram(program);

    #[cfg(debug_assertions)]
    Trash::dump_errors();

    // Get the link status
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    #[cfg(debug_assertions)]
    Trash::dump_errors();

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = vec![0 as GLchar; len as usize];

        gl::GetProgramInfoLog(
            program,
            len,
            ptr::null_mut(),
            buf.as_mut_ptr() as *mut GLchar,
        );

        error!("Failed to link shader");

        panic!("{:?}", CStr::from_ptr(buf.as_mut_ptr()));
    }
    Ok(program)
}

pub unsafe fn config_shader(program: GLuint, (name, data_type, size): (&str, GLenum, GLint)) {
    gl::UseProgram(program);
    #[cfg(debug_assertions)]
    Trash::dump_errors();

    let pos_attr = gl::GetAttribLocation(program, CString::new(name).unwrap().into_raw());
    #[cfg(debug_assertions)]
    Trash::dump_errors();

    #[cfg(feature = "shader_log")]
    {
        log::debug!("Shader Attribute {} <{}={}>", program, name, pos_attr);
    }

    gl::EnableVertexAttribArray(pos_attr as GLuint);
    #[cfg(debug_assertions)]
    Trash::dump_errors();
    gl::VertexAttribPointer(
        pos_attr as GLuint,
        size,
        data_type,
        gl::FALSE as GLboolean,
        0,
        ptr::null_mut(),
    );
    #[cfg(debug_assertions)]
    Trash::dump_errors();
}

pub unsafe fn assign_uniform4fv(
    program: GLuint,
    uniform: String,
    count: i32,
    data: Matrix4<f32>,
) {
    let loc = gl::GetUniformLocation(program, CString::new(uniform).unwrap().as_ptr());
    gl::UniformMatrix4fv(
        loc,
        count,
        gl::FALSE,
        mem::transmute(glm::value_ptr(&data).as_ptr()),
    );
}

pub unsafe fn assign_uniform2fv(
    program: GLuint,
    uniform: String,
    count: i32,
    data: Matrix2<f32>,
) {
    let loc = gl::GetUniformLocation(program, CString::new(uniform).unwrap().as_ptr());
    gl::UniformMatrix2fv(
        loc,
        count,
        gl::FALSE,
        mem::transmute(glm::value_ptr(&data).as_ptr()),
    );
}
