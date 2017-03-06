//
// kotetuco, 2017
//

use rgb::RGB;
use gba_color::GBAColor;

pub struct Graphics {
    vram_address: u32,
    screen_x: u16,
    screen_y: u16,
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            vram_address: 0x06000000,
            screen_x: 240,
            screen_y: 160,
        }
    }

    pub fn draw_dot(&self, x:u16, y:u16, color:&RGB) {
        let offset: u32 = ((y * self.screen_x) + x) as u32;
        let vram: *mut u16 = (self.vram_address + (offset * 2)) as *mut u16;
        unsafe {
            *vram = color.convert_u16_color();
        }
    }

    pub fn draw_box(&self, x:u16, y:u16, width:u16, height:u16, color:&RGB) {
        for offset_y in 0..height {
            for offset_x in 0..width {
                let valid_x = if x + offset_x > self.screen_x { self.screen_x } else { x + offset_x };
                let valid_y = if y + offset_y > self.screen_y { self.screen_y } else { y + offset_y };
                self.draw_dot(valid_x, valid_y, color);
            }
        }
    }

    pub fn width(&self) -> u16 {
        return self.screen_x;
    }

    pub fn height(&self) -> u16 {
        return self.screen_y;
    }
}
