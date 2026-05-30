use bootloader_api::info::{FrameBufferInfo, PixelFormat};

mod fonts;

const CHAR_PAD: usize = 1;

pub struct GraphicsManager {
    info: FrameBufferInfo,
    buffer: &'static mut [u8],
    scaling: usize,
    background_color: (u8, u8, u8),
    char_space: (usize, usize),
    cursor: (usize, usize),
}

impl GraphicsManager {
    pub fn new(info: FrameBufferInfo, buffer: &'static mut [u8]) -> Self {
        // color_definitions (R,G,B)
        let background_color = (30, 30, 36);

        // determine scaling amount
        let scaling;

        if info.width <= 720 {
            scaling = 1;
        } else if info.width <= 1440 {
            scaling = 2;
        } else {
            scaling = 3;
        }

        // how many charecters can fit on display
        let char_space = (info.width / (9 * scaling), info.height / (17 * scaling));
        let cursor = (0, 0);

        Self {
            info,
            buffer,
            scaling,
            background_color,
            char_space,
            cursor,
        }
    }

    pub fn wipe(&mut self) {
        self.fill_buffer(self.background_color);
    }

    fn fill_buffer(&mut self, color: (u8, u8, u8)) {
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

    pub fn write_text(&mut self, fg: (u8, u8, u8), text: &str) {
        if text.is_ascii() {
            let ascii_bytes = text.as_bytes();
            for c in ascii_bytes.iter() {

                match c {
                    b'\n' => {
                        self.cursor_newline();
                    }
                    _ => {
                        let bitmap = fonts::get_charecter_bitmap(*c);
                        self.blit_charecter(self.cursor_to_pixel_coords(), bitmap, fg);
                        self.cursor_right();
                    }
                }
            }
        }
    }

    /* blits a bitmap with offset */
    fn blit_charecter(&mut self, offset: (usize, usize), bitmap: [u8; 16], fg: (u8, u8, u8)) {
        for (row_local_i, row_value) in bitmap.iter().enumerate() {
            let row_global_i = offset.1 + (row_local_i) * self.scaling;
            for column_local_i in 0..8 {
                let column_global_i = offset.0 + (column_local_i * self.scaling);
                let pixel = ((row_value >> (7 - column_local_i)) & 1) != 0;
                let color = if pixel { fg } else { self.background_color };
                for x in 0..self.scaling {
                    for y in 0..self.scaling {
                        self.set_pixel_color(column_global_i + x, row_global_i + y, color);
                    }
                }
            }
        }
    }

    fn set_pixel_color(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        let width = self.info.stride;
        let bpp = self.info.bytes_per_pixel;

        let pixel_i = ((y * width) + x) * bpp;

        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.buffer[pixel_i] = color.0;
                self.buffer[pixel_i + 1] = color.1;
                self.buffer[pixel_i + 2] = color.2;
            }
            PixelFormat::Bgr => {
                self.buffer[pixel_i] = color.2;
                self.buffer[pixel_i + 1] = color.1;
                self.buffer[pixel_i + 2] = color.0;
            }
            _ => {
                // Error: Pixel format not supported
            }
        }
    }

    /* needs to change to jump to text but thats a later problem */
    fn cursor_left(&mut self) {
        if self.cursor.0 == 0 {
            self.cursor.1 -= 1;
            self.cursor.0 = self.char_space.1;
        } else {
            self.cursor.0 -= 1;
        }
    }

    fn cursor_right(&mut self) {
        self.cursor.0 += 1;
        if self.cursor.0 >= self.char_space.0 {
            self.cursor_newline();
        }
    }

    fn cursor_newline(&mut self) {
        self.cursor.0 = 0;
        if self.cursor.1 == self.char_space.1 -1 {
            // Chop the first bit of the framebuffer off to make room
            self.chop_frame_buffer(17*self.scaling);
            self.cursor.1 = self.char_space.1 - 1;
        } else {
            self.cursor.1 += 1;
        }
    }

    fn cursor_to_pixel_coords(&self) -> (usize, usize) {
        (
            (self.cursor.0 * (9 * self.scaling)),
            self.cursor.1 * (17 * self.scaling),
        )
    }

    /* Chops that many rows from the start of the frame buffer to make room for more text */
    fn chop_frame_buffer(&mut self, rows: usize) {
        let bpp = self.info.bytes_per_pixel;
        let offset = rows * self.info.stride * bpp;
        let buffer_size = self.buffer.len();
        let end = buffer_size - offset;

        self.buffer.copy_within(offset..buffer_size, 0);

        for i in (end..buffer_size).step_by(bpp) {
            match self.info.pixel_format {
                PixelFormat::Rgb => {
                    self.buffer[i] = self.background_color.0;
                    self.buffer[i + 1] = self.background_color.1;
                    self.buffer[i + 2] = self.background_color.2;
                }
                PixelFormat::Bgr => {
                    self.buffer[i] = self.background_color.2;
                    self.buffer[i + 1] = self.background_color.1;
                    self.buffer[i + 2] = self.background_color.0;
                }
                _ => {}
            }
        }
    }
}
