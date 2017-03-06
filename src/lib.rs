//
// kotetuco, 2017
//

#![feature(lang_items)]
#![no_std]
#![feature(asm)]
#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;
extern crate rlibc;

mod rgb;
mod gba_color;
mod graphics;

use rgb::RGB;
use rgb::RGBDef;
use graphics::Graphics;

#[no_mangle]
pub extern "C" fn entry() {
    init_graphic();

    let graphics: Graphics = Graphics::new();
    graphics.draw_box(20, 20, 100, 100, &RGB::light_red());
    graphics.draw_box(70, 50, 100, 100, &RGB::light_green());
    graphics.draw_box(120, 80, 100, 100, &RGB::light_blue());

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

#[allow(private_no_mangle_fns)]
#[no_mangle]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}
