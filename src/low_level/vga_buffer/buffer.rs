use super::Color;
use core::mem::swap;
pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Char {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}
impl Char {
    pub fn invert_colors(&mut self) {
        self.color_code.invert();
    }
}
#[repr(transparent)]

#[derive(Clone, Copy)]
pub struct Buffer {
    pub chars: [BufferRow; BUFFER_HEIGHT],
}
#[derive(Clone, Copy)]
pub struct BufferRow {
    pub chars: [Char; BUFFER_WIDTH],
}
impl BufferRow {
    pub fn put_char(&mut self, mut column_position: usize, mut character: Char) {
        loop {
            swap(&mut character, &mut self.chars[column_position]);
            column_position += 1;
            //Do while loops, why arent you in rust
            if self.chars[column_position].ascii_character == 0 || column_position == BUFFER_WIDTH {
                break;
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);
impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        Self::generate(foreground as u8, background as u8)
    }
    fn generate(foreground: u8, background: u8) -> ColorCode {
        ColorCode((background) << 4 | (foreground))
    }
    pub fn get_colors(&self) -> (u8, u8) {
        (self.0 % 16u8, self.0 >> 4u8)
    }
    pub fn invert(&mut self) {
        let colors = self.get_colors();
        *self = Self::generate(colors.1, colors.0)
    }
}
