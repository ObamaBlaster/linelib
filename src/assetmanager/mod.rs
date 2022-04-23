//Some helper traits for the assetmanager crate
pub mod fragment_shader;
pub mod vertex_shader;
pub mod svg_entity;

#[macro_export]
macro_rules! asset_list {
    ($($name:expr => $i:ident => $ref:ident),*,) => {
        pub struct AssetTable<'a>{
            $(
                pub $ref: Handle<'a, $i>,
            )*
        }



        fn load_assets<'a>(w:&'a AssetCache) -> AssetTable<'a>{

            $(
                let $ref = w.load::<$i>($name).unwrap().clone();
            )*


            AssetTable{
                $(
                    $ref:$ref,
                )*
            }
        }

    }
}

#[macro_export]
macro_rules! shader_list {
    ($($i:ident => $vs:ident => $fs:ident),*,) => {
        #[derive(Copy, Clone)]
        pub struct ShaderTable{
            $(
                pub $i: GLuint,
            )*
        }


        fn link_shader_table<'a>(w:&'a AssetTable) -> ShaderTable{

            $(
                let vs = w.$vs.read().shader;
                let fs = w.$fs.read().shader;
                let $i = unsafe {
                    link_program(
                        vs,
                        fs,
                    )
                    .unwrap()
                };

                // let $ref = w.load::<$i>($name).unwrap().clone();
            )*


            ShaderTable{
                $(
                    $i,
                )*
            }
        }

    }
}
