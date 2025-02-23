use core::ptr;

pub struct Framebuffer {
    pub base_address: *mut u32, // Pointer to the base address of the linear framebuffer
    pub width: usize,           // Width of the framebuffer in pixels
    pub height: usize,          // Height of the framebuffer in pixels
    pub bg_color: u32,          // Background color
}

pub const SCALE_FACTOR_X: usize = 1;
pub const SCALE_FACTOR_Y: usize = 1;

impl Framebuffer {
    // Create a new framebuffer instance
    pub fn new(base_address: u32, width: usize, height: usize) -> Self {
        Self {
            base_address: base_address as *mut u32,
            width: width / SCALE_FACTOR_X,
            height: height / SCALE_FACTOR_Y,
            bg_color: 0x111111,
        }
    }

    // Draw a pixel at (x, y) with the specified color
    pub fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            unsafe {
                for row in 0..SCALE_FACTOR_Y {
                    for col in 0..SCALE_FACTOR_X {
                        let physical_width = self.width * SCALE_FACTOR_X; // logical width multiplied by 4 gives the physical width
                        let offset = (y * SCALE_FACTOR_Y + row) * physical_width
                            + (x * SCALE_FACTOR_X + col);
                        ptr::write_volatile(self.base_address.add(offset), color);
                    }
                }
            }
        }
    }

    // Draw a rectangle at (x, y) with the specified width, height, and color
    pub fn draw_rectangle(&self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for i in 0..width {
            for j in 0..height {
                self.draw_pixel(x + i, y + j, color);
            }
        }
    }

    pub fn draw_image(&self, x: usize, y: usize, width: usize, height: usize, contents: &[u32]) {
        for i in 0..width {
            for j in 0..height {
                let color = contents[j * width + i];
                if (color & 0xFF000000) != 0 {
                    self.draw_pixel(x + i, y + j, color);
                }
            }
        }
    }

    pub fn get_center_xy(&self, width: usize, height: usize) -> (usize, usize) {
        let x = (self.width - width) / 2;
        let y = (self.height - height) / 2;
        (x, y)
    }
}
