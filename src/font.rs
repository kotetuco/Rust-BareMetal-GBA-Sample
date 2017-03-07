//
// kotetuco, 2017
//

use font_def::FONT_DATAS;

pub struct Font;

impl Font {
    pub fn get_charactor(ch: char) -> [u8; 16] {
        let index = ch as usize;
        return FONT_DATAS[index];
    }
}
