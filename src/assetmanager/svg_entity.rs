use crate::linevg::ast::Svg;
use crate::linevg::SVGTokenizer;
use crate::opengl::wrappers::Buffer::twodee_vertex_array_object;
use assets_manager::loader::Loader;
use assets_manager::Asset;
use assets_manager::BoxedError;
use gl::types::*;
use lyon::lyon_tessellation::{
    BuffersBuilder, StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers,
};
use lyon::math::point;
use lyon::path::path::Builder;
use lyon::path::Path;
use std::borrow::Cow;
use xmltree::Element;

#[derive(Debug, Clone)]
pub struct SvgEntity {
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
    pub svg: Svg,
    pub verts: Vec<f32>,
    pub indices : Vec<u32>,
}

impl Asset for SvgEntity {
    const EXTENSION: &'static str = "svg";

    // The serialization format (RON)
    type Loader = SvgEntityLoader;
}
#[derive(Copy, Clone, Debug)]
struct MyVertex {
    position: [f32; 2],
}
pub struct SvgEntityLoader;
impl Loader<SvgEntity> for SvgEntityLoader {
    fn load(content: Cow<[u8]>, id: &str) -> Result<SvgEntity, BoxedError> {
        let src = std::str::from_utf8(&content).unwrap();
        let mut svg_element = Element::parse(src.as_bytes()).unwrap();
        let svg = SVGTokenizer::parse_svg(&svg_element);
        println!("{:?}", svg);

        // TODO: Svg entities only support paths right now
        let mut iter = &svg.clone().unwrap();

        //TODO: This shoudl be replaced with a recursive method. It's almost done for testing, so too lazy to fix rn
        loop {
            match &iter {
                Svg::Root{children,..} => {
                    for s in children{
                        if let (Svg::Group{children,..}) = s {
                            for s in children{
                                println!("eee");
                                if let Svg::Path(path) = s {
                                    let (vao,vbo,ebo,v,i) = tessellate_stroke(path);
                                    return Ok(SvgEntity{
                                        vao,vbo,ebo,svg:svg.unwrap().clone(),verts:v,indices:i,
                                    });
                                }
                            }
                        }
                    }
                },
                _ => {continue; println!("asd");},
            }
        }

        // Transpile the paths with lyon here
        // Upload the verticies and indicies to the GPU

        // info!("Loaded \"{}\" svg entity", id);
    }
}

fn tessellate_stroke(path: &Path) -> (GLuint, GLuint, GLuint, Vec<f32>, Vec<u32>) {
    let mut options = StrokeOptions::default();
    let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();

    let mut tessellator = StrokeTessellator::new();
    options.line_width = 1.0;

    {
        // Compute the tessellation.
        tessellator
            .tessellate_path(
                path,
                &options,
                &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| MyVertex {
                    position: vertex.position().to_array(),
                }),
            )
            .unwrap();
    }

    let verts = geometry
        .vertices
        .clone()
        .into_iter()
        .fold(Vec::new(), |mut acc, f| {
            acc.push(f.position[0]);
            acc.push(f.position[1]);
            acc
        });

    let index = geometry
        .indices
        .clone()
        .into_iter()
        .fold(Vec::new(), |mut acc, f| {
            acc.push(f as u32);
            acc
        });

    let (vao, vbo, ebo) = twodee_vertex_array_object(verts.clone(), index.clone());
    (vao, vbo, ebo, verts, index)
}
