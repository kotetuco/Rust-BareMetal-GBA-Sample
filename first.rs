//
// kotetuco, 2016
//

#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern "C" fn entry() {
    loop {
    }
}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}
