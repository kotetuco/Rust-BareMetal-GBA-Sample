//
// kotetuco, 2017
//

use font_def::FONT_DATAS;

pub struct Font {
    size_width: u16,
    size_height: u16,
}

impl Font {
    pub fn new() -> Self {
        Font {
            size_width: 8,
            size_height: 16,
        }
    }

    pub fn get_character(&self, ch: char) -> [u8; 16] {
        let index = ch as usize;
        return FONT_DATAS[index];
    }

    pub fn font_width(&self) -> u16 {
        return self.size_width;
    }

    pub fn font_height(&self) -> u16 {
        return self.size_height;
    }
}
