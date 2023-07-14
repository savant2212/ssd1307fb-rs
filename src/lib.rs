use std::fmt;

use embedded_graphics_core::geometry::Dimensions;
use embedded_graphics_core::prelude::{Point, Size};
use embedded_graphics_core::primitives::Rectangle;
use embedded_graphics_core::Pixel;
use framebuffer::FramebufferError;

pub struct Display {
    fb: framebuffer::Framebuffer,
    frame: Vec<u8>,
    screen_buffer: Vec<u8>
}

impl Display {
    pub fn new(dev: &str) -> Result<Self, FramebufferError> {
        let fb = framebuffer::Framebuffer::new(dev)?;
        let rect_size = (&fb.var_screen_info.xres * &fb.var_screen_info.yres) as usize;
        let frame_size = fb.frame.len();
        let mut d = Display {
            fb: fb,
            frame: Vec::with_capacity(rect_size),
            screen_buffer: Vec::with_capacity(frame_size),
        };

        d.screen_buffer.resize(frame_size, 0);
        d.frame.resize(rect_size, 0);

        Ok(d)
    }

    pub fn flush(&mut self) -> Result<(), FramebufferError> {
        let mut i = 0;
        let mut j = 0;
        
        while i < self.frame.len() {
            self.screen_buffer[j] = self.frame[i] << 0
                | self.frame[i + 1] << 1
                | self.frame[i + 2] << 2
                | self.frame[i + 3] << 3
                | self.frame[i + 4] << 4
                | self.frame[i + 5] << 5
                | self.frame[i + 6] << 6
                | self.frame[i + 7] << 7;
            j += 1;
            i += 8;
        }
        self.fb.write_frame(&self.screen_buffer);
        Ok(())
    }
}
impl fmt::Debug for Display {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("ssd1307fd").field("fb", &self.fb).finish()
    }
}
impl Dimensions for Display {
    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            top_left: Point { x: 0, y: 0 },
            size: Size{ width: self.fb.var_screen_info.xres, height: self.fb.var_screen_info.yres },

        }
    }
}

impl embedded_graphics_core::draw_target::DrawTarget for Display {
    type Color = embedded_graphics_core::pixelcolor::BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let dm = self.bounding_box();

        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (127,63)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            if coord.x >= dm.size.width as i32  || coord.x < 0 || coord.y >= dm.size.height as i32 || coord.x < 0 { continue }
            // Calculate the index in the framebuffer.
            let index  = ((dm.size.width-1) as i32 - coord.x) + (coord.y * (dm.size.width as i32));
            self.frame[index as usize] = color.is_on() as u8;

        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
