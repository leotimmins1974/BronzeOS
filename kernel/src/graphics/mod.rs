use bootloader_api::info::{FrameBufferInfo, PixelFormat};

pub struct GraphicsManager {
    info: FrameBufferInfo,
    buffer: &'static mut [u8],
}

impl GraphicsManager {
    pub fn new(info: FrameBufferInfo, buffer: &'static mut [u8]) -> Self {
        Self { info, buffer }
    }

    pub fn fill_buffer(&mut self, color: &(u8, u8, u8)) {
        for i in (0..self.info.byte_len).step_by(self.info.bytes_per_pixel) {
            // determine the pixelformat
            match self.info.pixel_format {
                PixelFormat::Rgb => {
                    self.buffer[i] = color.0;
                    self.buffer[i + 1] = color.1;
                    self.buffer[i + 2] = color.2;
                }
                PixelFormat::Bgr => {
                    self.buffer[i] = color.2;
                    self.buffer[i + 1] = color.1;
                    self.buffer[i + 2] = color.0;
                }
                _ => {
                    // Error: Pixel format not supported
                }
            }
        }
    }
}
