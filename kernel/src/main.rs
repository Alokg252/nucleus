#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler] // custom panic handler
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // don't mangle (change) the name
/*
    extern C is ABI for similar binary conversion like C does so that it can
    communicate to other binaries written in C
*/
pub extern "C" fn _start() -> ! {
    loop {}
}
