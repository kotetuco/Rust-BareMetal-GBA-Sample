//
// kotetuco, 2017
//

use rgb::RGB;
use gba_color::GBAColor;
use font::Font;

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

    pub fn draw_circle(&self, center_x:u16, center_y:u16, r:u16, color:&RGB) {
        let mut x: u16 = r;
        let mut y: u16 = 0;
        let mut f: i32 = 3 - ((r as i32) * 2);

        // draw center
        self.draw_dot(center_x, center_y, color);

        loop {
            if x < y {
                break;
            }

            self.draw_dot(center_x + x, center_y + y, color);
            self.draw_dot(center_x - x, center_y + y, color);
            self.draw_dot(center_x + x, center_y - y, color);
            self.draw_dot(center_x - x, center_y - y, color);
            self.draw_dot(center_x + y, center_y + x, color);
            self.draw_dot(center_x - y, center_y + x, color);
            self.draw_dot(center_x + y, center_y - x, color);
            self.draw_dot(center_x - y, center_y - x, color);

            if f >= 0 {
                x -= 1;
                f -= (x * 4) as i32;
            }
            y += 1;
            f += (4 * y + 2) as i32;
        }
    }

    pub fn draw_char(&self, ch:char, x:u16, y:u16, color:&RGB) {
        let char_data:[u8; 16] = Font::get_charactor(ch);
        for index in 0..15 {
            let byte_data:u8 = char_data[index];
            let offset_y = index as u16;
            if (byte_data & 0x80) != 0x00 { self.draw_dot(x + 0, y + offset_y, color); }
            if (byte_data & 0x40) != 0x00 { self.draw_dot(x + 1, y + offset_y, color); }
            if (byte_data & 0x20) != 0x00 { self.draw_dot(x + 2, y + offset_y, color); }
            if (byte_data & 0x10) != 0x00 { self.draw_dot(x + 3, y + offset_y, color); }
            if (byte_data & 0x08) != 0x00 { self.draw_dot(x + 4, y + offset_y, color); }
            if (byte_data & 0x04) != 0x00 { self.draw_dot(x + 5, y + offset_y, color); }
            if (byte_data & 0x02) != 0x00 { self.draw_dot(x + 6, y + offset_y, color); }
            if (byte_data & 0x01) != 0x00 { self.draw_dot(x + 7, y + offset_y, color); }
        }
    }

    pub fn width(&self) -> u16 {
        return self.screen_x;
    }

    pub fn height(&self) -> u16 {
        return self.screen_y;
    }
}
