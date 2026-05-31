#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader_api::BootInfo;
use core::fmt::Write;
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::graphics::GraphicsManager;
use crate::keyboard::KeyboardManager;
use crate::time::TimeManager;

mod arch;
mod graphics;
mod keyboard;
mod memory;
mod time;

bootloader_api::entry_point!(entry);

lazy_static! {
    static ref DISPLAY: Mutex<Option<GraphicsManager>> = Mutex::new(None);
    static ref KEYBOARD: Mutex<Option<KeyboardManager>> = Mutex::new(None);
    static ref TIME: Mutex<Option<TimeManager>> = Mutex::new(None);
}

/* bronze kernel entry point */
fn entry(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();

        let gm = graphics::GraphicsManager::new(info, buffer);

        *DISPLAY.lock() = Some(gm);
    }

    wipe_buffer();
    println!("BRONZE KERNEL");
    println!("MADE BY LEO TIMMINS");
    println!("version 0.1.0");
    println!("-------------------");

    /* Keyboard Manager setup */
    print!("setting up keyboard manager...");

    let km = KeyboardManager::new();
    *KEYBOARD.lock() = Some(km);

    println!("success!");

    /* TimeManager setup */
    print!("setting up time manager...");

    let tm = TimeManager::new(0);
    *TIME.lock() = Some(tm);

    println!("success!");

    /* IDT setup */
    print!("setting up idt...");

    arch::init_idt();

    println!("success!");

    /* PIT setup */
    print!("setting up pit...");

    arch::init_pit();

    println!("success!");

    /* Memory setup */
    print!("setting up memory...");
    //not implemented
    println!("fail: not implemented");

    //temporary debugging view
    println!();
    println!("--- Bootloader Info ---");
    println!("boodloader api ver:     {:?}", boot_info.api_version);
    println!("kernel adr:             {:?}", boot_info.kernel_addr);
    println!(
        "kernel img offset:      {:?}",
        boot_info.kernel_image_offset
    );
    println!("kernel len:             {:?}", boot_info.kernel_len);
    println!(
        "kernel stack bottom:    {:?}",
        boot_info.kernel_stack_bottom
    );
    println!("kernel stack len:       {:?}", boot_info.kernel_stack_len);
    println!("mem regions:            {:?}", boot_info.memory_regions);
    println!(
        "phys mem offset:        {:?}",
        boot_info.physical_memory_offset
    );
    println!("ramdisk adr:            {:?}", boot_info.ramdisk_addr);
    println!("ramdisk len:            {:?}", boot_info.ramdisk_len);
    println!("recurs index:           {:?}", boot_info.recursive_index);
    println!("rsdp adr:               {:?}", boot_info.rsdp_addr);
    println!("tls template:           {:?}", boot_info.tls_template);
    println!("-----------------------");
    //endtemp

    println!("all tasks complete...");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!();
    println!("! KERNEL PANIC !");
    println!("something went wrong");
    println!("please restart your computer");
    loop {}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!("\n");
    }};
}

fn _print(args: core::fmt::Arguments) {
    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.write_fmt(args).unwrap();
    }
}

fn wipe_buffer() {
    if let Some(gm) = DISPLAY.lock().as_mut() {
        gm.wipe();
    }
}
