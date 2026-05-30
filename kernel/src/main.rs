/*

Bronze Kernel
By Leo Timmins, 2026

*/

#![no_std]
#![no_main]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;

mod graphics; // provides ascii bitmaps

bootloader_api::entry_point!(entry);

/* bronze kernel entry point */
fn entry(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();

        let mut gm = graphics::GraphicsManager::new(info, buffer);

        // color_definitions (R,G,B)
        let color_orange = (217, 137, 052);

        gm.fill_buffer(&color_orange);
    }

    loop {}
}

/* kernel panic handler */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
