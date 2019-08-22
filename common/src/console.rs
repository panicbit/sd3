use core::ptr::{read_volatile, write_volatile};
use core::fmt;

pub struct Console<'a> {
    buf: &'a mut [[u8; 3]],
    width: usize,
    height: usize,
    x_pos: usize,
    y_pos: usize,
    bg_color: [u8; 3],
    fg_color: [u8; 3],
}

impl<'a> Console<'a> {
    pub fn new(buf: &'a mut [[u8; 3]], width: usize, height: usize) -> Self {
        let width = width / 8;
        let height = height / 8;

        Self {
            buf,
            width,
            height,
            x_pos: 0,
            y_pos: 0,
            bg_color: [0; 3],
            fg_color: [255; 3],
        }
    }

    pub fn go_to(&mut self, x: usize, y: usize) {
        self.x_pos = x;
        self.y_pos = y;
    }

    pub fn set_fg(&mut self, color: [u8; 3]) {
        self.fg_color = color;
    }

    pub fn set_bg(&mut self, color: [u8; 3]) {
        self.bg_color = color;
    }

    pub fn clear(&mut self, mut color: [u8; 3]) {
        self.x_pos = 0;
        self.y_pos = 0;

        color.reverse();

        for pixel in &mut self.buf[..] {
            unsafe {
                write_volatile(pixel, color);
            }
        }
    }

    pub fn write_str(&mut self, str: &str) {
        self.write(str.as_bytes());
    }

    pub fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
    }

    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.write_newline();
            return;
        }

        if self.x_pos >= self.width {
            self.write_newline();
        }

        self.render_char(self.x_pos, self.y_pos, byte);

        self.x_pos += 1;
    }

    fn write_newline(&mut self) {
        self.y_pos += 1;
        self.x_pos = 0;

        if self.y_pos >= self.height {
            self.shift_buffer_up();
            self.y_pos -= 1;
        }
    }

    fn shift_buffer_up(&mut self) {
        for y in 8..(8 * self.height) {
            for x in 0..(8 * self.width) {
                let from = self.pos(x, y);
                let to = self.pos(x, y - 8);

                unsafe {
                    write_volatile(&mut self.buf[to], read_volatile(&self.buf[from]));
                }
            }
        }

        for y_off in 0..8 {
            for x in 0..(8 * self.width) {
                let to = self.pos(x, 8 * self.height - 1 - y_off);

                unsafe {
                    write_volatile(&mut self.buf[to], [0, 0, 0]);
                }
            }
        }
    }

    fn render_char(&mut self, x: usize, y: usize, ch: u8) {
        use font8x8::unicode::BASIC_UNICODE;

        if x >= self.width || y >= self.height {
            return;
        }

        let x = 8 * x;
        let y = 8 * y;

        let glyph = match BASIC_UNICODE.get(ch as usize) {
            None => return self.render_char(x, y, 0),
            Some(glyph) => glyph,
        };

        for (y_off, row) in glyph.byte_array().iter().copied().enumerate() {
            let y = y + y_off;

            for x_off in 0..8u8 {
                let x = x + x_off as usize;
                let luminance = (row >> x_off) & 1;
                let color = match luminance {
                    0 => self.bg_color,
                    _ => self.fg_color,   
                };

                self.blit(x, y, color);
            }
        }
    }

    fn blit(&mut self, x: usize, y: usize, color: [u8; 3]) {
        let pos = self.pos(x, y);

        if pos >= self.buf.len() {
            return;
        }

        unsafe {
            write_volatile(&mut self.buf[pos][0], color[2]);
            write_volatile(&mut self.buf[pos][1], color[1]);
            write_volatile(&mut self.buf[pos][2], color[0]);
        }
    }

    fn pos(&self, x: usize, y: usize) -> usize {
        let y = 8 * self.height - 1 - y;
        let pos = 8 * self.height * x + y;
        pos
    }
}

impl<'a> fmt::Write for Console<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}
