#![feature(trait_alias)]
#![feature(fn_traits)]
#![feature(concat_idents)]

#[macro_use]
pub extern crate simple_log;


use crate::libstates::State;
use std::collections::{LinkedList};
pub mod libstates;
pub mod linevg;
pub mod opengl;
pub mod assetmanager;
pub mod physics;
pub mod timers;
pub extern crate gl;
pub extern crate serde;
pub mod linein;
pub use imgui_opengl_renderer;

pub use lyon;

#[macro_export]
pub use bincode;

pub use xmltree;
pub use rapier2d;
pub use assets_manager;
pub use anyhow;
pub use serde_json;
pub use nalgebra_glm as glm;
pub use nalgebra;
pub use imgui_sdl2;
pub use crossbeam;
pub use threadpool;
pub use libp2p;
pub use futures;
pub use async_std;
pub use imgui;
pub use backtrace;

// Games are state machines
// Their only job is to render the state
// The state can be changed by the game however it wants
// In linelib's case we change it via a multithreaded timer


// Doesn't follow the Unity ECS model
pub type GameObject = dyn Fn(); 

#[derive(Clone)]
/// The MasterBuffer is basically a queue for OpenGL Calls
/// It's meant to help with debugging graphics pipelines
/// as well as creating framebuffers when the gamestate is updating slower than the render loop
pub struct MasterBuffer<GL_CALLS> 
where GL_CALLS : Fn()
{
    buffer : LinkedList<LinkedList<GL_CALLS>>
}

impl<GL_CALLS : Fn()> MasterBuffer<GL_CALLS>
{
    pub fn new() -> Self {
        let mut l = LinkedList::new();
        l.push_front(LinkedList::new());
        Self{
            buffer: l,
        }
    }
    pub fn buffer_gl_front(&mut self, calls: GL_CALLS){
        self.buffer.front_mut().unwrap().push_front(calls);
    }

    pub fn buffer_gl_back(&mut self, calls: GL_CALLS){
        self.buffer.front_mut().unwrap().push_back(calls);
    }
    pub fn next_frame(&mut self){
        self.buffer.push_back(LinkedList::new());
    }
    pub fn render_gl(&mut self){
        for obj in self.buffer.pop_front().unwrap().iter(){
            obj();
        }
    }
}


// Simple helper wrapper to stop the main file from being so phat
// Kinda like descriptor classes

pub trait GameWrapper<A>{
    fn new(args: A) -> Self;
    fn tick(&mut self, args : A);
}

pub use renderdoc;