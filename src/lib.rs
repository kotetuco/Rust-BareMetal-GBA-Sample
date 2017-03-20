//
// kotetuco, 2017
//

#![feature(lang_items)]
#![no_std]

mod rgb;
mod gba_color;
mod graphics;
mod font;
mod font_def;

use rgb::RGB;
use rgb::RGBDef;
use graphics::Graphics;

#[no_mangle]
pub extern "C" fn entry() {
    init_graphic();

    let graphics: Graphics = Graphics::new();

    // rectangle
    graphics.draw_box(20, 20, 100, 100, &RGB::light_red());
    graphics.draw_box(70, 50, 100, 100, &RGB::light_green());
    graphics.draw_box(120, 80, 100, 100, &RGB::light_blue());

    // circle
    let radius:u16 = 20;
    graphics.draw_circle(graphics.width() - (radius + 1), (radius + 1), radius, &RGB::light_yellow());

    // character
    graphics.draw_string("Hello, World!", 0, 0, &RGB::white());

    loop {}
}

fn init_graphic() {
    let ioram_address: u32 = 0x04000000;
    unsafe {
        let video_mode: *mut u8 = ioram_address as *mut u8;
        *video_mode = 0x03; // mode 3
        let bg: *mut u8 = (ioram_address + 1) as *mut u8;
        *bg = 0x04; // BG2
    }
}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt() -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}
