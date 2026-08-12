#[allow(dead_code)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
impl Color {
    pub const BLACK: Color = Color { r: 10, g: 2, b: 18 };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const PURPLE_AURA: Color = Color {
        r: 147,
        g: 51,
        b: 234,
    };
    pub const DARK_PURPLE: Color = Color {
        r: 58,
        g: 12,
        b: 92,
    };
    pub const NEON_VIOLET: Color = Color {
        r: 216,
        g: 180,
        b: 254,
    };
    pub const DRAGON_EYE: Color = Color {
        r: 0,
        g: 255,
        b: 220,
    };
}

#[allow(dead_code)]
pub struct FramebufferConsole {
    pub base_ptr: *mut u32,
    pub width: usize,
    pub height: usize,
    pub pixels_per_scanline: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

#[allow(dead_code)]
impl FramebufferConsole {
    pub unsafe fn new(
        base_ptr: *mut u32,
        width: usize,
        height: usize,
        pixels_per_scanline: usize,
    ) -> Self {
        Self {
            base_ptr,
            width,
            height,
            pixels_per_scanline,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// Draw a single pixel on screen
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height || self.base_ptr.is_null() {
            return;
        }

        let pixel_val = ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        let offset = y * self.pixels_per_scanline + x;
        unsafe {
            self.base_ptr.add(offset).write_volatile(pixel_val);
        }
    }

    /// Render Radial Purple Aura Glow across the entire screen
    pub fn render_purple_aura(&mut self) {
        let center_x = self.width / 2;
        let center_y = self.height / 2;
        let max_dist = (center_x * center_x + center_y * center_y) as f32;

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = (x as isize - center_x as isize) as f32;
                let dy = (y as isize - center_y as isize) as f32;
                let dist_sq = dx * dx + dy * dy;

                // Gradient factor calculation for soft purple aura
                let factor = 1.0 - (dist_sq / (max_dist * 0.65)).min(1.0);

                let r = (10.0 + factor * 137.0) as u8;
                let g = (2.0 + factor * 49.0) as u8;
                let b = (18.0 + factor * 216.0) as u8;

                self.draw_pixel(x, y, Color { r, g, b });
            }
        }
    }

    /// Render Cyber Purple Dragon Mascot Emblem
    pub fn render_purple_dragon(&mut self) {
        let start_x = if self.width > 120 {
            self.width / 2 - 20
        } else {
            10
        };
        let start_y = if self.height > 100 {
            self.height / 2 - 20
        } else {
            10
        };

        // 16x16 Pixel Bitmap representation of A.A.O Cyber Dragon
        let dragon_sprite: [u16; 16] = [
            0b0000011000110000,
            0b0000111101111000,
            0b0001111111111100,
            0b0011011111110110, // Eyes at bits
            0b0111111111111110,
            0b1111111111111111,
            0b1111110000111111,
            0b1111000000001111,
            0b0111000000001110,
            0b0011100000011100,
            0b0001110000111000,
            0b0000111001110000,
            0b0000011111100000,
            0b0000001111000000,
            0b0000000110000000,
            0b0000000000000000,
        ];

        let scale = 4; // Scale up dragon pixel size

        for row in 0..16 {
            let line = dragon_sprite[row];
            for col in 0..16 {
                if (line & (1 << (15 - col))) != 0 {
                    let color = if row == 3 && (col == 4 || col == 11) {
                        Color::DRAGON_EYE // Glowing Cyan Eyes
                    } else if row < 4 {
                        Color::NEON_VIOLET
                    } else {
                        Color::PURPLE_AURA
                    };

                    for sy in 0..scale {
                        for sx in 0..scale {
                            self.draw_pixel(
                                start_x + col * scale + sx,
                                start_y + row * scale + sy,
                                color,
                            );
                        }
                    }
                }
            }
        }
    }
}

pub static mut FRAMEBUFFER: Option<FramebufferConsole> = None;

pub fn init_graphics(base_ptr: *mut u32, width: usize, height: usize, scanline: usize) {
    unsafe {
        let mut console = FramebufferConsole::new(base_ptr, width, height, scanline);
        console.render_purple_aura();
        console.render_purple_dragon();
        FRAMEBUFFER = Some(console);
    }
}
