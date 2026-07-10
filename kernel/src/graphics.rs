use limine::framebuffer::Framebuffer;

pub fn put_pixel(
    framebuffer: &Framebuffer,
    x: usize,
    y: usize,
    color: u32,
) {
    let pitch = framebuffer.pitch() as usize;
    let bytes_per_pixel = (framebuffer.bpp() / 8) as usize;

    unsafe {
        let pixel = framebuffer
            .addr()
            .cast::<u8>()
            .add(y * pitch + x * bytes_per_pixel)
            .cast::<u32>();

        pixel.write_volatile(color);
    }
}

pub fn draw_box(
    framebuffer: &Framebuffer,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
) {
    for dy in 0..height {
        for dx in 0..width {
            put_pixel(framebuffer, x + dx, y + dy, color);
        }
    }
}