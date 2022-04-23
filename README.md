# linelib
Rust sdl2 and imgui wrapper (with other junk too)

# Getting Started


Cargo.toml

```
[dependencies]
linelib = {git = "https://github.com/ObamaBlaster/linelib"}

...


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
