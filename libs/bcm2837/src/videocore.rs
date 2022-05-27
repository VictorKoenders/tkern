use core::{cell::UnsafeCell, ptr::NonNull};

use bcm2837_pac::VCMAILBOX;

pub struct VideoCore {
    mb: VCMAILBOX,
}

mod tags {
    pub const REQUEST: u32 = 0;
}

impl VideoCore {
    pub fn new(mb: VCMAILBOX) -> Self {
        Self { mb }
    }

    pub fn get_firmware_revision(&mut self) -> Result<u32> {
        let result = self.send(Command::Properties, &[tags::REQUEST, 0x0000_0001])?;
        Ok(result[2])
    }

    pub fn get_board_model(&mut self) -> Result<u32> {
        let result = self.send(Command::Properties, &[tags::REQUEST, 0x0001_0001])?;
        Ok(result[2])
    }

    pub fn get_board_revision(&mut self) -> Result<u32> {
        let result = self.send(Command::Properties, &[tags::REQUEST, 0x0001_0002])?;
        Ok(result[2])
    }

    pub fn get_board_serial(&mut self) -> Result<u64> {
        let result = self.send(Command::Properties, &[tags::REQUEST, 0x0001_0004])?;
        Ok(((result[5] as u64) << 32) | (result[6] as u64))
    }

    pub fn get_command_line(&mut self) -> Result<[u32; 36]> {
        self.send(Command::Properties, &[tags::REQUEST, 0x0005_0001])
    }

    pub fn framebuffer_init(&mut self, width: u32, height: u32) -> Result<FrameBuffer> {
        let address = unsafe {
            let new_inner = FrameBufferInitInner {
                width,
                height,
                vwidth: width,
                vheight: height,
                ..Default::default()
            };
            core::ptr::write_volatile(FRAME_BUFFER_INIT.inner.get(), new_inner);
            &FRAME_BUFFER_INIT as *const _ as usize as u32
        };
        let address = 0x4000_0000 + address + Command::FrameBuffer as u32;
        while self.mb.status0.read().full().bit_is_set() {
            cortex_a::asm::nop();
        }

        unsafe {
            self.mb.write.write_with_zero(|w| w.bits(address));
        }
        loop {
            while self.mb.status0.read().empty().bit_is_set() {
                cortex_a::asm::nop();
            }
            if self.mb.read.read().bits() == Command::FrameBuffer as _ {
                return Ok(unsafe {
                    core::ptr::read_volatile(FRAME_BUFFER_INIT.inner.get()).into_buffer()
                });
            }
        }
    }

    pub fn send(&mut self, command: Command, data: &[u32]) -> Result<[u32; 36]> {
        let mut buffer = [0u32; BUFFER_LEN];
        // Length of this buffer
        buffer[0] = BUFFER_LEN as u32 * 4;
        // buffer[1] = 0; // process request (already zeroed)
        buffer[2..][..data.len()].copy_from_slice(data);
        // buffer[2+data.len()..] = 0; // end tag (already zeroed)

        while self.mb.status0.read().full().bit_is_set() {
            cortex_a::asm::nop();
        }

        unsafe { BUFFER.write(buffer) };
        let ptr = BUFFER.ptr_command(command);
        unsafe {
            self.mb.write.write_with_zero(|w| w.bits(ptr));
        }

        loop {
            while self.mb.status0.read().empty().bit_is_set() {
                cortex_a::asm::nop();
            }
            if self.mb.read.read().bits() == ptr {
                return Ok(unsafe { BUFFER.read() });
            }
        }
    }
}

pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}

const BUFFER_LEN: usize = 36;
#[repr(C, align(16))]
struct VideoCoreBuffer {
    _do_not_access_me: UnsafeCell<[u32; BUFFER_LEN]>,
}

static BUFFER: VideoCoreBuffer = VideoCoreBuffer {
    _do_not_access_me: UnsafeCell::new([0u32; BUFFER_LEN]),
};

impl VideoCoreBuffer {
    /// Get the address that needs to be written to [`VCMAILBOX`]'s `write` register.
    ///
    /// This will be a mix of `self.ptr()` with the lower 4 bits set to the `u8` value in `command`
    fn ptr_command(&self, command: Command) -> u32 {
        let ptr = self._do_not_access_me.get() as usize as u32;
        // assert bottom 4 bytes are 0'd out
        debug_assert_eq!(ptr & 0xf, 0);

        let command = command as u8 as u32;
        // and that the top 4 bytes are 0'd out
        debug_assert_eq!(command & 0xf0, 0);

        ptr | command
    }

    /// Safety: don't use the write() and read() methods from multiple threads
    unsafe fn write(&self, buffer: [u32; BUFFER_LEN]) {
        // Write the buffer
        core::ptr::write_volatile(self._do_not_access_me.get(), buffer);
    }

    /// Safety: don't use the write() and read() methods from multiple threads
    unsafe fn read(&self) -> [u32; BUFFER_LEN] {
        // Read the entire buffer
        core::ptr::read_volatile(self._do_not_access_me.get())
    }
}
/// Safety: see write() and read()
unsafe impl Sync for VideoCoreBuffer {}

#[repr(u8)]
pub enum Command {
    FrameBuffer = 0x01,
    Properties = 0x08,
}

#[repr(C, align(16))]
struct FrameBufferInit {
    inner: UnsafeCell<FrameBufferInitInner>,
}

#[repr(C)]
struct FrameBufferInitInner {
    width: u32,
    height: u32,
    vwidth: u32,
    vheight: u32,
    bytes: u32,
    depth: u32,
    ignorex: u32,
    ignorey: u32,
    pointer: u32,
    size: u32,
}

impl FrameBufferInitInner {
    const fn new() -> Self {
        Self {
            width: 800,
            height: 480,
            vwidth: 800,
            vheight: 480,
            bytes: 0,
            depth: 24,
            ignorex: 0,
            ignorey: 0,
            pointer: 0,
            size: 0,
        }
    }

    fn into_buffer(&self) -> FrameBuffer {
        FrameBuffer {
            width: self.width,
            height: self.height,
            pitch: self.width * self.depth / 8,
            bytes_per_pixel: self.depth / 8,
            pointer: NonNull::new((self.pointer - 0x4000_0000) as *mut _)
                .expect("Could not allocate framebuffer"),
        }
    }
}

impl Default for FrameBufferInitInner {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for FrameBufferInit {}

static FRAME_BUFFER_INIT: FrameBufferInit = FrameBufferInit {
    inner: UnsafeCell::new(FrameBufferInitInner::new()),
};

#[derive(Debug)]
pub struct FrameBuffer {
    width: u32,
    height: u32,

    pitch: u32,
    bytes_per_pixel: u32,

    pointer: NonNull<u8>,
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        red: 255,
        green: 255,
        blue: 255,
    };

    fn as_bytes(&self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

impl FrameBuffer {
    pub fn put_pixel(&self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        // 0x0000_0000_7c10_0000
        unsafe {
            let addr = self
                .pointer
                .as_ptr()
                .offset((y * self.pitch + x * self.bytes_per_pixel) as isize)
                .cast();
            core::ptr::write_volatile(addr, color.as_bytes());
        }
    }

    pub fn text(&self, x: u32, y: u32, text: &str) {
        use font8x8::UnicodeFonts;

        let mut x = x;
        for c in text.chars() {
            let bytes = font8x8::BASIC_FONTS
                .get(c)
                .unwrap_or_else(|| font8x8::BASIC_FONTS.get('?').unwrap());
            for (dy, bits) in bytes.into_iter().enumerate() {
                for dx in 0..8 {
                    if bits & (1 << dx) > 0 {
                        self.put_pixel(x + dx, y + dy as u32, Color::WHITE);
                    }
                }
            }
            x += 8;
        }
    }
}
