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
use crate::keyboard::KeyboardManager;

mod arch;
mod graphics;
mod keyboard;
mod time;

bootloader_api::entry_point!(entry);

lazy_static! {
    static ref DISPLAY: Mutex<Option<GraphicsManager>> = Mutex::new(None);
    static ref KEYBOARD: Mutex<Option<KeyboardManager>> = Mutex::new(None);
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
        gm.write_text(ORANGE, "BRONZE KERNEL\n".as_bytes());
        gm.write_text(WHITE, "MADE BY LEO TIMMINS\n".as_bytes());
        gm.write_text(WHITE, "version 0.1.0\n\n".as_bytes());
    }

    /* Keyboard Manager setup */
    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_text(WHITE, "setting up keyboard manager...\n".as_bytes());
    }
    let km = KeyboardManager::new();
    *KEYBOARD.lock() = Some(km);
    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_text(GREEN, "success!\n\n".as_bytes());
    }

    /* Interupts setup */
    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_text(WHITE, "setting up interrupts...\n".as_bytes());
    }
    arch::init();
    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_text(GREEN, "success!\n\n".as_bytes());
    }

    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_text(GREEN, "all tasks complete...\n".as_bytes());
    }

    loop {}
}

/* kernel panic handler */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_text(RED, "\n! KERNEL PANIC !\n".as_bytes());
        gm.write_text(RED, "something went wrong\n".as_bytes());
        gm.write_text(YELLOW, _info.message().as_str().unwrap().as_bytes());
        gm.write_text(WHITE, "\nplease restart your computer".as_bytes());
    }
    loop {}
}
