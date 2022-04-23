use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use lyon::lyon_tessellation::{FillOptions, StrokeOptions, FillTessellator};

use crate::GameObject;

pub trait DrawableVector {
    fn draw(&self);
    fn add_point(&mut self);
    fn remove_point(&mut self);
}

#[derive(Copy, Clone)]
pub enum TessOption {
    Stroke(StrokeOptions),
    Fill(FillOptions),
}


pub fn svg_to_gl(svg_path: &'_ str) -> Box<GameObject> {
    Box::new(|| {})
}

pub mod ast {
    use lyon::path::Path;

    #[derive(Debug)]
    pub enum PathCmd {
        MoveTo(usize, usize),
        MoveTo_d(usize, usize),

        LineTo(usize, usize),
        LineTo_d(usize, usize),

        VertLineTo(usize, usize),
        VertLineTo_d(usize, usize),

        HorizLineTo(usize, usize),
        HorizLineTo_d(usize, usize),

        CubicBez(usize, usize, usize, usize, usize, usize),
        CubicBez_d(usize, usize, usize, usize, usize, usize),

        SmoothCubicBez(usize, usize, usize, usize, usize),
        SmoothCubicBez_d(usize, usize, usize, usize, usize),

        QuadBez(usize, usize, usize, usize),
        QuadBez_d(usize, usize, usize, usize),

        SmoothQuadBez(usize, usize),
        SmoothQuadBez_d(usize, usize),

        Arc(usize, usize, usize, usize, usize, usize, usize),
        Arc_d(usize, usize, usize, usize, usize, usize, usize),
    }

    #[derive(Debug, Clone)]
    pub enum Svg {
        // Width, Height, ViewBox, Children
        Root {
            width: usize,
            height: usize,
            viewbox: (f32, f32, f32, f32),
            children: Vec<Svg>,
        },
        // ID, children
        Group {
            id: String,
            children: Vec<Svg>,
        },
        Path (
            Path
        ),
        Unsupported,
    }
}

// TODO: double check this transpiler. Rendering errors can arrise from here

// Create an AST from a small subset of the SVG standard
pub mod SVGTokenizer {
    use crate::linevg::ast::PathCmd;
    use crate::linevg::ast::Svg;
    use crate::linevg::Quad;
    use lyon::geom::euclid::vec2;
    use lyon::geom::point;
    use lyon::path::builder::WithSvg;
    use lyon::path::path::Builder;
    use lyon::path::traits::SvgPathBuilder;
    use lyon::path::Path;
    use std::sync::atomic::AtomicUsize;
    use xmltree::Element;
    use lyon::lyon_svg::path_utils::build_path;

    fn parse_children(element: &Element) -> Result<Vec<Svg>, String> {
        // Find the children
        let mut c = Vec::new();
        for e in &element.children {
            let e = e.as_element().unwrap();
            c.push(parse_svg(e)?);
        }
        Ok(c)
    }

    fn parse_path_commands(cmds: &String) -> Svg {
        // let tokens = cmds.split(" ");
        let mut builder = Path::builder().with_svg();
        let mut path = build_path(builder,cmds).unwrap();
        // parse_path(tokens.collect::<Vec<&str>>().as_slice(), &mut builder);
        // builder.close();
        Svg::Path(path)
    }

    // fn parse_path(tokens: &[&str], builder: &mut WithSvg<Builder>) {
    //     if tokens.len() == 0 {
    //         return
    //     }

    //     let cmd = tokens[0];
    //     match cmd {
    //         "M" => {
    //             let pt = tokens[1];

    //             let args = tokens[1].split(",").collect::<Vec<&str>>();
    //             let args = vec![args[0].parse::<f32>().unwrap(), args[1].parse().unwrap()];

    //             builder.move_to(point(args[0], args[1]));

    //             parse_path(&tokens[2..], builder);
    //         }
    //         "m" => {
    //             let pt = tokens[1];

    //             let args = tokens[1].split(",").collect::<Vec<&str>>();
    //             let args = vec![args[0].parse::<f32>().unwrap(), args[1].parse().unwrap()];

    //             builder.relative_move_to(vec2(args[0], args[1]));

    //             parse_path(&tokens[2..], builder);
    //         }

    //         "L" => {
    //             let pt = tokens[1];

    //             let args = tokens[1].split(",").collect::<Vec<&str>>();
    //             let args = vec![args[0].parse::<f32>().unwrap(), args[1].parse().unwrap()];

    //             builder.line_to(point(args[0], args[1]));

    //             parse_path(&tokens[2..], builder);
    //         }

    //         "l" => {
    //             let pt = tokens[1];

    //             let args = tokens[1].split(",").collect::<Vec<&str>>();
    //             let args = vec![args[0].parse::<f32>().unwrap(), args[1].parse().unwrap()];

    //             builder.relative_line_to(vec2(args[0], args[1]));

    //             parse_path(&tokens[2..], builder);
    //         }
    //         "H" => {
    //             let x = tokens[1].parse().unwrap();
    //             builder.horizontal_line_to(x);
    //             parse_path(&tokens[2..], builder);
    //         }
    //         "h" => {
    //             let x = tokens[1].parse().unwrap();
    //             builder.relative_horizontal_line_to(x);
    //             parse_path(&tokens[2..], builder);
    //         }
    //         "V" => {
    //             let x = tokens[1].parse().unwrap();
    //             builder.vertical_line_to(x);
    //             parse_path(&tokens[2..], builder);
    //         }
    //         "v" => {
    //             let x = tokens[1].parse().unwrap();
    //             builder.relative_vertical_line_to(x);
    //             parse_path(&tokens[2..], builder);
    //         }

    //         "C" => {
    //             let pt1 = tokens[1].split(",").collect::<Vec<&str>>();
    //             let pt2 = tokens[2].split(",").collect::<Vec<&str>>();
    //             let pt3 = tokens[3].split(",").collect::<Vec<&str>>();

    //             builder.cubic_bezier_to(
    //                 point(pt1[0].parse().unwrap(), pt1[1].parse().unwrap()),
    //                 point(pt2[0].parse().unwrap(), pt2[1].parse().unwrap()),
    //                 point(pt3[0].parse().unwrap(), pt3[1].parse().unwrap()),
    //             );

    //             parse_path(&tokens[4..], builder);
    //         },

    //         "c" => {
    //             let pt1 = tokens[1].split(",").collect::<Vec<&str>>();
    //             let pt2 = tokens[2].split(",").collect::<Vec<&str>>();
    //             let pt3 = tokens[3].split(",").collect::<Vec<&str>>();

    //             builder.relative_cubic_bezier_to(
    //                 vec2(pt1[0].parse().unwrap(), pt1[1].parse().unwrap()),
    //                 vec2(pt2[0].parse().unwrap(), pt2[1].parse().unwrap()),
    //                 vec2(pt3[0].parse().unwrap(), pt3[1].parse().unwrap()),
    //             );

    //             parse_path(&tokens[4..], builder);
    //         },
    //         "S" => {
    //             let pt1 = tokens[1].split(",").collect::<Vec<&str>>();
    //             let pt2 = tokens[2].split(",").collect::<Vec<&str>>();

    //             builder.smooth_cubic_bezier_to(
    //                 point(pt1[0].parse().unwrap(), pt1[1].parse().unwrap()),
    //                 point(pt2[0].parse().unwrap(), pt2[1].parse().unwrap()),
    //             );

    //             parse_path(&tokens[3..], builder);
    //         },
    //         "s" => {
    //             let pt1 = tokens[1].split(",").collect::<Vec<&str>>();
    //             let pt2 = tokens[2].split(",").collect::<Vec<&str>>();

    //             builder.smooth_relative_cubic_bezier_to(
    //                 vec2(pt1[0].parse().unwrap(), pt1[1].parse().unwrap()),
    //                 vec2(pt2[0].parse().unwrap(), pt2[1].parse().unwrap()),
    //             );

    //             parse_path(&tokens[3..], builder);
    //         }

    //         _ => {
    //             panic!("@ THE DISCO")
    //         }
    //     }
    // }

    pub fn parse_svg(element: &Element) -> Result<Svg, String> {
        match element.name.to_lowercase().as_str() {
            "svg" => {
                let c = parse_children(element)?;
                let width = element
                    .attributes
                    .get("width")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let height = element
                    .attributes
                    .get("height")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                println!("{:?}", element.attributes.get("viewBox"));
                let viewBox = element
                    .attributes
                    .get("viewBox")
                    .unwrap()
                    .parse::<Quad>()
                    .unwrap();

                Ok(Svg::Root {
                    width,
                    height,
                    viewbox: viewBox.into(),
                    children: c,
                })
            }
            "g" => {
                static COUNTER: AtomicUsize = AtomicUsize::new(0);
                let id = {
                    if let Some(id) = element.attributes.get("id") {
                        id.to_owned()
                    } else {
                        let c = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        format!("linevg_grp{:?}", c.to_string())
                    }
                };

                let c = parse_children(element)?;

                Ok(Svg::Group { id, children: c })
            }

            "path" => {
                static COUNTER: AtomicUsize = AtomicUsize::new(0);
                let id = {
                    if let Some(id) = element.attributes.get("id") {
                        id.to_owned()
                    } else {
                        let c = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        format!("linevg_path{:?}", c.to_string())
                    }
                };

                let d = element.attributes.get("d").unwrap();

                Ok(parse_path_commands(d))
            }

            // A lot of the SVG spec will not be supported cuz work
            x => Ok(Svg::Unsupported),
        }
    }
}

pub mod SVGTranspiler {
    use std::collections::LinkedList;

    use crate::{GameObject};
    use xmltree::Element;
    fn path(path_dom: Element) -> Result<Box<GameObject>, String> {
        let d = path_dom.attributes.get("d");
        let path_lenth = path_dom.attributes.get("path_length");

        match d {
            Some(cmds) => {
                if let Some(p) = path_lenth {
                    warn!("asd");
                }

                let mut buffer: LinkedList<Box<GameObject>> = LinkedList::new();

                //

                Ok(Box::new(move || {
                    for obj in &buffer {
                        obj();
                    }
                }))
            }
            None => Err(format!("{:?} does not have a 'd' attribute", &path_dom)),
        }
    }
}

pub struct Quad(f32, f32, f32, f32);
impl FromStr for Quad {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr = s.split(" ").collect::<Vec<&str>>();
        println!("{:?}", arr);
        Ok(Quad(
            arr[0].parse::<f32>()?,
            arr[1].parse::<f32>()?,
            arr[2].parse::<f32>()?,
            arr[3].parse::<f32>()?,
        ))
    }
}

impl Into<(f32, f32, f32, f32)> for Quad {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.0, self.1, self.2, self.3)
    }
}


pub mod line;
pub mod animation;