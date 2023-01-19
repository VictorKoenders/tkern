use uefi::{
    proto::console::gop::{BltOp, BltPixel, GraphicsOutput},
    table::boot::ScopedProtocol,
};

pub struct Renderer<'gop> {
    gop: ScopedProtocol<'gop, GraphicsOutput<'gop>>,
}

impl<'gop> Renderer<'gop> {
    pub fn new(gop: ScopedProtocol<'gop, GraphicsOutput<'gop>>) -> Self {
        Self { gop }
    }

    pub fn block<'a>(
        &'a mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        inner: impl FnOnce(&mut BlockRenderer<'_, 'gop>),
    ) {
        let dimensions = self.gop.current_mode_info().resolution();
        let mut renderer = BlockRenderer {
            gop: &mut self.gop,
            x: 0,
            y: 0,
            width: dimensions.0,
            height: dimensions.1,
        };
        renderer.sub(x, y, width, height, inner);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const BLACK: Color = Color::new(0, 0, 0);

    // fn to_pixels(self, pixel_format: PixelFormat) -> [u8; 3] {
    //     match pixel_format {
    //         PixelFormat::Rgb => [self.r, self.g, self.b],
    //         PixelFormat::Bgr => [self.b, self.g, self.r],
    //         PixelFormat::Bitmask => unimplemented!(),
    //         PixelFormat::BltOnly => unimplemented!(),
    //     }
    // }
    fn to_blt(self) -> BltPixel {
        BltPixel::new(self.r, self.g, self.b)
    }
}

pub struct BlockRenderer<'parent, 'gop> {
    gop: &'parent mut GraphicsOutput<'gop>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl<'parent, 'gop> BlockRenderer<'parent, 'gop> {
    pub fn sub(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        inner: impl FnOnce(&mut BlockRenderer<'_, 'gop>),
    ) {
        let mut renderer = BlockRenderer {
            gop: self.gop,
            x,
            y,
            width,
            height,
        };
        renderer.draw_background(Color::BLACK);
        inner(&mut renderer);
        renderer.draw_outline(Color::WHITE);
    }

    fn draw_background(&mut self, color: Color) {
        let color = color.to_blt();
        self.gop
            .blt(BltOp::VideoFill {
                color,
                dest: (self.x + 1, self.y + 1),
                dims: (self.width - 2, self.height - 2),
            })
            .unwrap();
    }
    fn draw_outline(&mut self, color: Color) {
        let color = color.to_blt();
        // top border
        self.gop
            .blt(BltOp::VideoFill {
                color,
                dest: (self.x, self.y),
                dims: (self.width, 1),
            })
            .unwrap();
        // left border
        self.gop
            .blt(BltOp::VideoFill {
                color,
                dest: (self.x, self.y),
                dims: (1, self.height),
            })
            .unwrap();
        // bottom border
        self.gop
            .blt(BltOp::VideoFill {
                color,
                dest: (self.x, self.y + self.height - 1),
                dims: (self.width, 1),
            })
            .unwrap();
        // right border
        self.gop
            .blt(BltOp::VideoFill {
                color,
                dest: (self.x + self.width - 1, self.y),
                dims: (1, self.height),
            })
            .unwrap();
    }

    pub fn draw_text<'this, 'text>(
        &'this mut self,
        text: &'text str,
    ) -> TextBuilder<'this, 'text, 'parent, 'gop> {
        TextBuilder {
            parent: self,
            text,
            params: TextParams::default(),
        }
    }

    fn xy_to_index(&self, x: usize, y: usize) -> Option<usize> {
        let mode_info = self.gop.current_mode_info();
        if x >= mode_info.resolution().0 || y >= mode_info.resolution().1 {
            None
        } else {
            Some((x + y * mode_info.stride()) * 4)
        }
    }
}

fn get_font_char(char: char) -> Option<[u8; 8]> {
    let char = char as usize;
    if char < 128 {
        Some(font8x8::legacy::BASIC_LEGACY[char])
    } else {
        None
    }
}

pub struct TextBuilder<'parent, 'text, 'parent_parent, 'gop> {
    parent: &'parent mut BlockRenderer<'parent_parent, 'gop>,
    text: &'text str,
    params: TextParams,
}

impl TextBuilder<'_, '_, '_, '_> {
    pub fn at(mut self, x: usize, y: usize) -> Self {
        self.params.x = x;
        self.params.y = y;
        self
    }
}

impl Drop for TextBuilder<'_, '_, '_, '_> {
    fn drop(&mut self) {
        let set_pixel = self.params.color.to_blt();
        let clear_pixel = self.params.background_color.to_blt();

        let mut x = self.params.x + self.parent.x;
        let y = self.params.y + self.parent.y;

        let min_x = self.parent.x + 1;
        let min_y = self.parent.y + 1;
        let max_x = self.parent.x + self.parent.width - 1;
        let max_y = self.parent.y + self.parent.height - 1;
        for c in self.text.chars() {
            let char = get_font_char(c).unwrap_or_else(|| get_font_char('?').unwrap());
            for (offset_y, row) in char.into_iter().enumerate() {
                let pixel_y = y + offset_y;
                if pixel_y >= max_y || pixel_y <= min_y {
                    continue;
                }
                for mask in 0..8 {
                    let pixel_x = x + mask;
                    if pixel_x >= max_x || pixel_x <= min_x {
                        continue;
                    }
                    if let Some(index) = self.parent.xy_to_index(pixel_x, pixel_y) {
                        let bit_is_set = (row & (1 << mask)) != 0;
                        unsafe {
                            self.parent.gop.frame_buffer().write_value(
                                index,
                                if bit_is_set { set_pixel } else { clear_pixel },
                            );
                        }
                    }
                }
            }
            x += 8 + self.params.spacing;
            if x >= max_x {
                break;
            }
        }
    }
}

pub struct TextParams {
    pub x: usize,
    pub y: usize,
    pub spacing: usize,
    pub color: Color,
    pub background_color: Color,
}

impl Default for TextParams {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            spacing: 0,
            color: Color::WHITE,
            background_color: Color::BLACK,
        }
    }
}
