// THIS IS DEPRECATED UNTIL I FIND A WAY TO MAKE THIS SHIT WORK

use gl::*;
use gl::types::*;
use image::RgbaImage;
use image::Frame;
use image::gif::GifEncoder;

pub struct GifRecorder {
    encoder : GifEncoder<std::fs::File>,
    frames : Vec<Frame>,

    mem_size_limit : usize,
}

impl GifRecorder {
    /// Output file from the CWD and the limit to how many frames we can store
    /// Mem Size Limit : In bytes
    pub fn new(filename : String, mem_size_limit : usize) -> GifRecorder{
        let file = std::fs::File::create(&filename).unwrap();

        GifRecorder {
            encoder: GifEncoder::new_with_speed(file, 30),
            frames : Vec::new(),
            mem_size_limit : frame_limtit,
        }
    }

    pub fn start_recording() {

    }

    /// We should be destroying each instance of a Recorder after the recording ends
    /// This lets us set the frame_limit accordingly to how much memory is currently being used in a scene
    pub fn finish_recording(self) {
        self.encoder.encode_frame(frame).unwrap();
    }

    pub fn write_frame(&mut self, x : GLint, y : GLsizei, width : GLsizei, height : GLsizei) {

        // Get the rgba of the frame from opengl here
        let img = unsafe {
            let buffer = image::RgbaImage::new(width as u32, height as u32);
            let buffer = buffer.into_vec();
            let (buffer, length, capacity) = buffer.into_raw_parts();
            gl::ReadPixels(0, 0, width, height, gl::RGBA, gl::UNSIGNED_BYTE, buffer as *mut GLvoid);
            RgbaImage::from_vec(width as u32, height as u32, Vec::from_raw_parts(buffer, length, capacity)).unwrap()
        };

        let frame = Frame::new(img);

        self.frames.push(frame);

        // Check if we've used up all our memory
        // If we have, end the recording


    }
}
