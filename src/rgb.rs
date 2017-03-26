//
// kotetuco, 2017
//

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait RGBDef {
    fn black() -> Self;
    fn light_red() -> Self;
    fn light_green() -> Self;
    fn light_yellow() -> Self;
    fn light_blue() -> Self;
    fn light_purple() -> Self;
    fn light_pale_blue() -> Self;
    fn white() -> Self;
    fn light_gray() -> Self;
    fn dark_red() -> Self;
    fn dark_green() -> Self;
    fn dark_yellow() -> Self;
    fn dark_blue() -> Self;
    fn dark_purple() -> Self;
    fn dark_pale_blue() -> Self;
    fn dark_gray() -> Self;
}

impl RGBDef for RGB {
    fn black() -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0x00,};
    }

    fn light_red() -> RGB {
        return RGB { r: 0x1F, g: 0x00, b: 0x00,};
    }

    fn light_green() -> RGB {
        return RGB { r: 0x00, g: 0x1F, b: 0x00,};
    }

    fn light_yellow() -> RGB {
        return RGB { r: 0x1F, g: 0x1F, b: 0x00,};
    }

    fn light_blue() -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0x1F,};
    }

    fn light_purple() -> RGB {
        return RGB { r: 0x1F, g: 0x00, b: 0x1F,};
    }

    fn light_pale_blue() -> RGB {
        return RGB { r: 0x00, g: 0x1F, b: 0x1F,};
    }

    fn white() -> RGB {
        return RGB { r: 0x1F, g: 0x1F, b: 0x1F,};
    }

    fn light_gray() -> RGB {
        return RGB { r: 0x0F, g: 0x0F, b: 0x0F,};
    }

    fn dark_red() -> RGB {
        return RGB { r: 0x0F, g: 0x00, b: 0x00,};
    }

    fn dark_green() -> RGB {
        return RGB { r: 0x00, g: 0x0F, b: 0x00,};
    }

    fn dark_yellow() -> RGB {
        return RGB { r: 0x0F, g: 0x0F, b: 0x00,};
    }

    fn dark_blue() -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0x0F,};
    }

    fn dark_purple() -> RGB {
        return RGB { r: 0x0F, g: 0x00, b: 0x0F,};
    }

    fn dark_pale_blue() -> RGB {
        return RGB { r: 0x0F, g: 0x0F, b: 0x0F,};
    }

    fn dark_gray() -> RGB {
        return RGB { r: 0x07, g: 0x07, b: 0x07,};
    }
}
