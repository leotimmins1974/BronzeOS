/*

Bronze Kernel
By Leo Timmins, 2026

*/

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;

mod graphics; // provides ascii bitmaps
mod arch;

bootloader_api::entry_point!(entry);

/* bronze kernel entry point */
fn entry(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();

        let mut gm = graphics::GraphicsManager::new(info, buffer);

        let white = (240, 240, 240);
        let green = (100, 240, 100);
        let red = (240, 100, 100);

        gm.wipe();
        gm.write_text(white, "BRONZE OS\n");
        gm.write_text(white, "MADE BY LEO TIMMINS\n\n");
        
        gm.write_text(white, "setting up interupts...\n");
        arch::init();
        gm.write_text(green, "success!\n\n");

        gm.write_text(white, "all tasks complete");
    }

    loop {}
}

/* kernel panic handler */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
