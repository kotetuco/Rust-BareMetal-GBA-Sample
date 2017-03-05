//
// kotetuco, 2017
// 

#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]

#[no_mangle]
pub extern "C" fn entry() {
    init_graphic();

    let color:u16 = convert_u16_color(0, 255, 0);
    let vram_address:u32 = 0x06000000;

    for y in 0..160 {
        for x in 0..240 {
            let offset: u32 = ((y * 240) + x) as u32;
            let vram: *mut u16 = (vram_address + (offset * 2)) as *mut u16;
            unsafe {
                *vram = color;
            }
        }
    }

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

fn convert_u16_color(r:u8, g:u8, b:u8) -> u16{
    return (((b >> 3) as u16) << 10) + (((g >> 3) as u16) << 5) + (r >> 3) as u16;
}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}
