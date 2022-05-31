use volatile::Volatile;

pub struct FrameBuffer {
    buffer: Volatile<&'static mut [u8]>,
    width: u32,
    height: u32,
    depth: u32,
    pixel_order: PixelOrder,
    bits_per_pixel: u32,
    pixels_per_row: u32,
}

#[derive(Debug, Copy, Clone)]
pub(super) enum PixelOrder {
    BGR = 0x00,
    RGB = 0x01,
}

impl FrameBuffer {
    pub(super) fn new(
        addr: u32,
        len: u32,
        width: u32,
        height: u32,
        depth: u32,
        pixel_order: PixelOrder,
    ) -> Self {
        Self {
            buffer: Volatile::new(unsafe {
                core::slice::from_raw_parts_mut(addr as usize as *mut u8, len as usize)
            }),
            width,
            height,
            depth,
            pixel_order,
            bits_per_pixel: depth / 8,
            pixels_per_row: width * depth / 8,
        }
    }

    pub fn square(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        let (color, size) = self.color_to_slice(color);
        let color = &color[..size];
        if x >= self.width || y >= self.height {
            return;
        }
        let max_x = (x + width).min(self.width);
        let max_y = (y + height).min(self.height);
        for x in x..max_x {
            for y in y..max_y {
                let index = ((x * self.bits_per_pixel) + (y * self.pixels_per_row)) as usize;
                self.buffer
                    .index_mut(index..index + size)
                    .copy_from_slice(color);
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn scroll_up_by(&mut self, pixels: u32, clear_color: Color) {
        let start = (pixels * self.pixels_per_row) as usize;
        let end = (self.width * self.height * self.bits_per_pixel) as usize;
        self.buffer.copy_within(start..end, 0);
        self.square(0, self.height - pixels, self.width, pixels, clear_color);
    }

    pub fn char(&mut self, x: u32, y: u32, char: char, color: Color) {
        use font8x8::{unicode::BasicFonts, UnicodeFonts};

        let (color, size) = self.color_to_slice(color);
        let color = &color[..size];
        let font = BasicFonts::new();

        let glyph = font.get(char).unwrap_or(font.get('?').unwrap());
        for (dy, row) in glyph.into_iter().enumerate() {
            let mut index =
                ((x * self.bits_per_pixel) + ((y + dy as u32) * self.pixels_per_row)) as usize;
            for bit in 0..8 {
                let color = match row & (1 << bit) {
                    0 => &[0, 0, 0, 0][..size],
                    _ => color,
                };
                self.buffer
                    .index_mut(index..index + size)
                    .copy_from_slice(color);
                index += self.bits_per_pixel as usize;
            }
        }
    }

    fn color_to_slice(&self, color: Color) -> ([u8; 4], usize) {
        match (self.depth, self.pixel_order) {
            (24, PixelOrder::RGB) => ([color.r, color.g, color.b, 0], 4),
            (24, PixelOrder::BGR) => ([color.b, color.g, color.r, 0], 4),
            _ => {
                #[cold]
                fn do_panic(depth: u32, pixel_order: PixelOrder) -> ! {
                    panic!(
                        "Unknown depth and pixel order: {} and {:?}",
                        depth, pixel_order
                    );
                }
                do_panic(self.depth, self.pixel_order)
            }
        }
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
}
