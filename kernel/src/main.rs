#![no_std]
#![no_main]

use limine::request::MemoryMapRequest;

mod graphics;
mod memory;

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

#[used]
#[unsafe(link_section = ".requests")]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

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

    graphics::framebuffer::draw_box(&framebuffer, 200, 100, 300, 80, 0x00ff0000);
    graphics::framebuffer::draw_box(&framebuffer, 600, 200, 50, 200, 0x0000ff00);

    // usable free memory locations provided by limine
    let memory_map = MEMORY_MAP_REQUEST
        .get_response()
        .expect("No memory map from Limine");

    for entry in memory_map.entries() {
        let base = entry.base;
        let length = entry.length;
        let end = base + length;
        let kind = entry.entry_type;

        core::hint::spin_loop();
    }

    // testing bitmap working
    let mut bitmap = [0u8; 4];

    memory::bitmap::set(&mut bitmap, 13);
    memory::bitmap::set(&mut bitmap, 21);

    let page13 = memory::bitmap::is_set(&bitmap, 13);
    let page21 = memory::bitmap::is_set(&bitmap, 21);
    let page14 = memory::bitmap::is_set(&bitmap, 14);

    core::hint::spin_loop();

    loop {}
}
