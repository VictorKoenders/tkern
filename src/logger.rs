use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use conquer_once::spin::OnceCell;
use core::{
    fmt::{self, Write},
    ptr,
};
use font8x8::UnicodeFonts;
use spinning_top::Spinlock;

/// The global logger instance used for the `log` crate.
static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

/// A [`Logger`] instance protected by a spinlock.
struct LockedLogger(Spinlock<Logger>);

/// Additional vertical space between lines
const LINE_SPACING: usize = 0;
/// Additional vertical space between separate log messages
const LOG_SPACING: usize = 2;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn magenta() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 255,
        }
    }
    fn blue() -> Self {
        Self { r: 0, g: 0, b: 255 }
    }
    fn green() -> Self {
        Self { r: 0, g: 255, b: 0 }
    }
    fn yellow() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 0,
        }
    }
    fn red() -> Self {
        Self { r: 255, g: 0, b: 0 }
    }
    fn white() -> Self {
        Self { r: 255, g: 255, b: 255 }
    }
}

/// Initialize a text-based logger using the given pixel-based framebuffer as output.  
pub fn init(framebuffer: &'static mut FrameBuffer) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(framebuffer));
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Framebuffer info: {:?}", logger.info());
}

impl LockedLogger {
    /// Create a new instance that logs to the given framebuffer.
    pub fn new(framebuffer: &'static mut FrameBuffer) -> Self {
        LockedLogger(Spinlock::new(Logger::new(framebuffer)))
    }

    pub fn info(&self) -> FrameBufferInfo {
        self.0.lock().info()
    }

    /// Force-unlocks the logger to prevent a deadlock.
    ///
    /// This method is not memory safe and should be only used when absolutely necessary.
    pub unsafe fn force_unlock(&self) {
        self.0.force_unlock();
    }
}

impl log::Log for LockedLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let mut logger = self.0.lock();
        logger.set_color(match record.level() {
            log::Level::Trace => Color::magenta(),
            log::Level::Debug => Color::blue(),
            log::Level::Info => Color::green(),
            log::Level::Warn => Color::yellow(),
            log::Level::Error => Color::red(),
        });
        write!(logger, "{:^5}: ", record.level()).unwrap();
        logger.set_color(Color::white());
        writeln!(logger, "{}", record.args()).unwrap();
        logger.add_vspace(LOG_SPACING);
    }

    fn flush(&self) {}
}

/// Allows logging text to a pixel-based framebuffer.
struct Logger {
    framebuffer: &'static mut FrameBuffer,
    x_pos: usize,
    y_pos: usize,
    color: Color,
}

impl Logger {
    /// Creates a new logger that uses the given framebuffer.
    pub fn new(framebuffer: &'static mut FrameBuffer) -> Self {
        let mut logger = Self {
            framebuffer,
            x_pos: 0,
            y_pos: 0,
            color: Color::white(),
        };
        logger.clear();
        logger
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn newline(&mut self) {
        self.y_pos += 8 + LINE_SPACING;
        self.carriage_return()
    }

    fn add_vspace(&mut self, space: usize) {
        self.y_pos += space;
    }

    fn carriage_return(&mut self) {
        self.x_pos = 0;
    }

    /// Erases all text on the screen.
    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.framebuffer.buffer_mut().fill(0);
    }

    fn width(&self) -> usize {
        self.info().horizontal_resolution
    }

    fn height(&self) -> usize {
        self.info().vertical_resolution
    }

    fn info(&self) -> FrameBufferInfo {
        self.framebuffer.info()
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                if self.x_pos >= self.width() {
                    self.newline();
                }
                if self.y_pos >= (self.height() - 8) {
                    self.clear();
                }
                let rendered = font8x8::BASIC_FONTS
                    .get(c)
                    .expect("character not found in basic font");
                self.write_rendered_char(rendered);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: [u8; 8]) {
        for (y, byte) in rendered_char.iter().enumerate() {
            for (x, bit) in (0..8).enumerate() {
                let alpha = if *byte & (1 << bit) == 0 { 0 } else { 255 };
                self.write_pixel(self.x_pos + x, self.y_pos + y, alpha);
            }
        }
        self.x_pos += 8;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info().stride + x;
        let color = match self.info().pixel_format {
            PixelFormat::RGB => [intensity, intensity, intensity, 0],
            PixelFormat::BGR => [intensity, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            _ => unimplemented!(),
        };
        let color = [
            ((color[0] as u16) * (self.color.r as u16) / (u8::max_value() as u16)) as u8,
            ((color[1] as u16) * (self.color.g as u16) / (u8::max_value() as u16)) as u8,
            ((color[2] as u16) * (self.color.b as u16) / (u8::max_value() as u16)) as u8,
            color[3],
        ];

        let bytes_per_pixel = self.info().bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer.buffer_mut()[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer.buffer()[byte_offset]) };
    }
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
