#[macro_export]
macro_rules! read_to_string {
    ($file:expr $(,)?) => {
        std::fs::read_to_string(std::path::Path::new($file))
            .unwrap()
            .replace("\r", "")
    };
}

#[macro_export]
macro_rules! shader_path {
    ($assets:expr, $file:expr $(,)?) => {
        format!("{}/shaders/{}", $assets, $file)
    };
}

#[macro_export]
macro_rules! assets_path {
    ($file:expr $(,)?) => {
        pub const ASSETS: &str = $file;
    };
}

#[macro_export]
macro_rules! init_gl_gc {
    () => {
        lazy_static! {
            pub static ref GL_GC: Mutex<Vec<GLuint>> = Mutex::new(Vec::new());
        }
    };
}

#[macro_export]
macro_rules! init_sdl {
    ($title:expr, $width:expr, $height:expr) => {{
        let sdl = sdl2::init().unwrap();

        let video_subsystem = sdl.video().unwrap();
        let timer_subsystem = sdl.timer().unwrap();

        let window = video_subsystem
            .window($title, $width, $height)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        (sdl, window, video_subsystem, timer_subsystem)
    }};
}

#[macro_export]
macro_rules! init_gl {
    ($major:expr=>$minor:expr, $depth:expr, $video_subsystem:expr, $window:expr) => {
        let gl_attr = $video_subsystem.gl_attr();
        gl_attr.set_context_major_version($major);
        gl_attr.set_context_minor_version($minor);
        gl_attr.set_double_buffer(true);
        gl_attr.set_depth_size($depth);

        let _gl_context = $window.gl_create_context().unwrap();
        gl::load_with(|s| $video_subsystem.gl_get_proc_address(s) as *const c_void);
        $video_subsystem
            .gl_set_swap_interval(sdl2::video::SwapInterval::Immediate)
            .unwrap();
    };
}

#[macro_export]
macro_rules! init_imgui {
    ($video_subsystem:expr, $window:expr) => {{
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let mut handler = ImguiSdl2::new(&mut imgui, &$window);
        let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
            $video_subsystem.gl_get_proc_address(s) as _
        });

        (imgui, handler, renderer)
    }};
}
