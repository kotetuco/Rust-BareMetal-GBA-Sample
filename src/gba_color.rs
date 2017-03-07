//
// kotetuco, 2017
//

use rgb::RGB;

pub trait GBAColor {
    fn convert_u16_color(&self) -> u16;
}

impl GBAColor for RGB {
    fn convert_u16_color(&self) -> u16{
        return (((self.b >> 3) as u16) << 10) + (((self.g >> 3) as u16) << 5) + (self.r >> 3) as u16;
    }
}
