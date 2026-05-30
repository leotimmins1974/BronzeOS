/*

Bronze Kernel
By Leo Timmins, 2026

*/

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::graphics::GraphicsManager;

mod arch;
mod graphics; // provides ascii bitmaps

bootloader_api::entry_point!(entry);

lazy_static! {
    static ref DISPLAY: Mutex<Option<GraphicsManager>> = Mutex::new(None);
}

const WHITE: (u8, u8, u8) = (240, 240, 240);
const PURPLE: (u8, u8, u8) = (195, 0, 255);
const GREEN: (u8, u8, u8) = (0, 255, 85);
const RED: (u8, u8, u8) = (255, 25, 0);
const ORANGE: (u8, u8, u8) = (255, 140, 0);
const YELLOW: (u8, u8, u8) = (255, 208, 0);

/* bronze kernel entry point */
fn entry(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();

        let mut gm = graphics::GraphicsManager::new(info, buffer);

        *DISPLAY.lock() = Some(gm);
    }

    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.wipe();
        gm.write_text(ORANGE, "BRONZE KERNEL\n");
        gm.write_text(WHITE, "MADE BY LEO TIMMINS\n\n");

        gm.write_text(WHITE, "setting up interrupts...\n");
    }

    arch::init();

    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_text(GREEN, "success!\n\n");
    }

    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_text(GREEN, "all tasks complete\n");
    }

    panic!("test message");

    loop {}
}

/* kernel panic handler */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_text(RED, "\n! KERNEL PANIC !\n");
        gm.write_text(RED, "something went wrong\n");
        gm.write_text(YELLOW, _info.message().as_str().unwrap());
        gm.write_text(WHITE, "\nplease restart your computer");
    }
    loop {}
}
