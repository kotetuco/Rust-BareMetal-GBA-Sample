//
// kotetuco, 2016
// 

#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![no_std]
#![feature(asm)]

#[no_mangle]
pub extern "C" fn entry() {
    loop {
    }
}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {}
}

//#[no_mangle]
#[lang = "eh_personality"]
pub extern fn eh_personality() {}
