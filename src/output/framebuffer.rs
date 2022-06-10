use utils::atomic_mutex::AtomicMutex;
use videocore_mailbox::{Color, FrameBuffer};

pub(super) static FRAMEBUFFER: AtomicMutex<Option<FrameBufferOutput>> = AtomicMutex::new(None);

pub fn init(buffer: FrameBuffer) {
    *FRAMEBUFFER.lock() = Some(FrameBufferOutput::new(buffer));
}

pub struct FrameBufferOutput {
    buffer: FrameBuffer,
    x: u32,
    y: u32,
    start_x: u8,
    spacing_x: u8,
    spacing_y: u8,
    clear_color: Color,
    color: Color,
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
            clear_color: Color::BLACK,
            color: Color::WHITE,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn reset_color(&mut self) {
        self.color = Color::WHITE;
    }

    fn newline(&mut self) {
        let spacing_y = u32::from(self.spacing_y);
        if self.y + spacing_y >= self.buffer.height() {
            self.buffer.scroll_up_by(spacing_y, self.clear_color);
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
                self.buffer.char(self.x, self.y, char, self.color);
                self.x += u32::from(self.spacing_x);
                if self.x + u32::from(self.spacing_x) > self.buffer.width() {
                    self.newline();
                }
            }
        }
        Ok(())
    }
}
