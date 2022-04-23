use std::{
    sync::{atomic::AtomicUsize},
};

use gl::types::*;

// Search 'delete' in http://docs.gl/
// Just something to count for GPU memory leaks
// Only supporting features that are in both gl3 and gl4

// These are used for reference counting
pub static VERTEX_ARRAYS: AtomicUsize = AtomicUsize::new(0);
pub static BUFFERS: AtomicUsize = AtomicUsize::new(0);
pub static FRAMEBUFFERS: AtomicUsize = AtomicUsize::new(0);
pub static RENDERBUFFERS: AtomicUsize = AtomicUsize::new(0);

pub static SHADERS: AtomicUsize = AtomicUsize::new(0);
pub static PROGRAMS: AtomicUsize = AtomicUsize::new(0);

static SAMPLERS: AtomicUsize = AtomicUsize::new(0);
static TEXTURES: AtomicUsize = AtomicUsize::new(0);

pub mod macros;
pub mod wrappers;
