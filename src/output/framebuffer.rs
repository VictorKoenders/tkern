use videocore_mailbox::{Color, FrameBuffer};

pub(crate) struct FrameBufferOutput {
    buffer: FrameBuffer,
    x: u32,
    y: u32,
    start_x: u8,
    spacing_x: u8,
    spacing_y: u8,
}

impl FrameBufferOutput {
    pub fn new(buffer: FrameBuffer) -> Self {
        Self {
            buffer,
            x: 2,
            y: 2,
            start_x: 2,
            spacing_x: 8,
            spacing_y: 10,
        }
    }
    fn newline(&mut self) {
        let spacing_y = u32::from(self.spacing_y);
        if self.y + spacing_y >= self.buffer.height() {
            self.buffer.scroll_up_by(spacing_y, Color::BLACK);
        } else {
            self.y += spacing_y;
        }
        self.x = u32::from(self.start_x);
    }
}

impl core::fmt::Write for FrameBufferOutput {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            if char == '\r' {
                self.x = u32::from(self.start_x);
            } else if char == '\n' {
                self.newline();
            } else {
                self.buffer.char(self.x, self.y, char, Color::WHITE);
                self.x += u32::from(self.spacing_x);
                if self.x + u32::from(self.spacing_x) > self.buffer.width() {
                    self.newline();
                }
            }
        }
        Ok(())
    }
}
