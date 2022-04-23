# linelib
Rust sdl2 and imgui wrapper (with other junk too)

# Getting Started


Cargo.toml

```
[dependencies]
linelib = {git = "https://github.com/ObamaBlaster/linelib"}

...

# from https://github.com/Rust-SDL2/rust-sdl2
[dependencies.sdl2]
version = "0.35"
default-features = false
features = ["ttf","image","gfx","mixer","static-link","use-vcpkg"]

[package.metadata.vcpkg]
dependencies = ["sdl2[x11]", "sdl2-image[libjpeg-turbo,tiff,libwebp]", "sdl2-ttf", "sdl2-gfx", "sdl2-mixer"]
git = "https://github.com/microsoft/vcpkg"
rev = "9eb76edabf4b1cb1abfb67823d900daf8112da0f"

[package.metadata.vcpkg.target]
x86_64-pc-windows-msvc = { triplet = "x64-windows-static-md" }

```

Command line
```
cargo install cargo-vcpkg
cargo vcpkg build
cargo build

```

main.rs
```
// For a SDL2 window
use sdl2::video::GLProfile;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 0);
    let window = video_subsystem
        .window("Window", 1920, 1080)
        .opengl()
        .build()
        .unwrap();

    'running: loop {
        window.gl_swap_window();
    }
}

```
