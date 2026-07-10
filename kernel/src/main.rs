#![no_std]
#![no_main]

use limine::{
    request::FramebufferRequest,
    BaseRevision,
};

use core::panic::PanicInfo;

#[panic_handler] // custom panic handler
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// When Limine loads the ELF, it scans for this .requests section.
// "This kernel would like a framebuffer."
// If it can satisfy that request, it prepares one and writes the response before jumping to _start().
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(no_mangle)] // don't mangle (change) the name
/*
    extern C is ABI for similar binary conversion like C does so that it can
    communicate to other binaries written in C
*/
pub extern "C" fn _start() -> ! {

    let framebuffer_response = FRAMEBUFFER_REQUEST
        .get_response()
        .expect("Limine did not provide a framebuffer");

    let framebuffer = framebuffer_response
        .framebuffers()
        .next()
        .expect("No framebuffer provided");
    let fb_ptr = framebuffer.addr().cast::<u32>();

    unsafe {
        // Pixel (0, 0) = White (ARGB/XRGB 0xFFFFFFFF)
        fb_ptr.write_volatile(0xFFFF_FFFF);
        (fb_ptr.wrapping_add(4)).write_volatile(0xFFFF_FFFF);
        (fb_ptr.wrapping_add(16)).write_volatile(0xFFFF_FFFF);
    }

    loop {}
}
