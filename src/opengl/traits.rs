pub trait RenderTarget {

}

pub trait Render {
    pub fn render(target : RenderTarget) -> Result<()>;
}

pub trait Mesh {
    pub fn gl_init() -> Result<()>;
    pub fn gl_update() -> Result<()>;
}