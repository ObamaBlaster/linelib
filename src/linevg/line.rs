use gl::types::*;
use lyon::lyon_tessellation::{FillOptions, FillTessellator, BuffersBuilder, FillVertex, VertexBuffers, StrokeTessellator, StrokeOptions, StrokeVertex};
use nalgebra_glm::Vec2;

use crate::opengl::wrappers::Buffer::twodee_vertex_array_object;

use super::TessOption;

#[derive(Copy, Clone, Debug)]
struct MyVertex { position: [f32; 2]}


// Wraps a lyon path with a vao number
pub struct Path {
    pub path: lyon::path::Path,
    pub vao: GLuint,
    pub count: i32,
}

impl Path {
    // Upload the vertices and indices to the GPU with a fill tessaltor
    pub fn fill_tess(path : &lyon::path::Path, d: &mut FillTessellator, fill_options: &FillOptions) -> (GLuint, i32) {
        let mut geometry: VertexBuffers<MyVertex, u32> = VertexBuffers::new();
        {
            // Compute the tessellation.
            d 
                .tessellate_path(
                    path,
                    &FillOptions::default(),
                    &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| MyVertex {
                        position: vertex.position().to_array(),
                    }),
                )
                .unwrap();
        }
        let mut v =  Vec::new();
        for c in geometry.vertices.clone() {
            v.push(c.position[0]);
            v.push(c.position[1]);
        }

        let (vao, vbo, ebo) = twodee_vertex_array_object(v, geometry.indices.clone());

        let count = ((geometry.indices).len()) as i32;
        (vao, count)
    }


    // Upload the vertices and indices to the GPU with a stroke tessaltor
    pub fn stroke_tess(path : &lyon::path::Path, d: &mut StrokeTessellator, fill_options: &StrokeOptions) -> (GLuint, i32) {
        let mut geometry: VertexBuffers<MyVertex, u32> = VertexBuffers::new();
            // Compute the tessellation.
            d 
                .tessellate_path(
                    path,
                    fill_options,
                    &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| MyVertex {
                        position: vertex.position().to_array(),
                    }),
                )
                .unwrap();
        let mut v =  Vec::new();
        for c in geometry.vertices.clone() {
            v.push(c.position[0]);
            v.push(c.position[1]);
        }

        let (vao, vbo, ebo) = twodee_vertex_array_object(v, geometry.indices.clone());
        let count = ((geometry.indices).len()) as i32;

        // println!("{:?}", geometry.vertices);
        // println!("{:?}", geometry.indices);
        (vao, count)
    }

    pub fn new(p: lyon::path::Path, tess : &TessOption, stroke_tess: &mut StrokeTessellator, fill_tess : &mut FillTessellator) -> Self {
        if let TessOption::Fill(fill) = tess {
            let (vao, count) = Path::fill_tess(&p, fill_tess, fill);
            return Path { path: p, count: count, vao: vao };
        } else if let TessOption::Stroke(stroke) = tess {
            let (vao, count) = Path::stroke_tess(&p, stroke_tess,stroke);
            return Path { path: p, count: count, vao: vao };
        }else{
            panic!("This is logically impossible");
        }
    }
}
