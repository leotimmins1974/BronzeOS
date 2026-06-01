#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader_api::config::Mapping;
use bootloader_api::{BootInfo, BootloaderConfig};
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

bootloader_api::entry_point!(entry, config = &BOOTLOADER_CONFIG);

const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

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
    println!();

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

    /* hardware init */
    println!();
    println!("---- hardware initialization ----");

    arch::init(
        &boot_info.memory_regions,
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("phys_mem_offset was None"),
    );

    println!();

    #[cfg(debug_assertions)]
    {
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
    }

    println!("---------------------------------");
    println!("all tasks complete...");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!();
    println!("! KERNEL PANIC !");
    println!("@ {}", _info.location().unwrap());
    println!("> {}", _info.message());
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
