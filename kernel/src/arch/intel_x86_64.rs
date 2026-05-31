// for intels x86_64 architecture

use pic8259::ChainedPics;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::PURPLE;

pub enum InterruptIndex {
    Breakpoint = 3,
    Timer = 32,
    Keyboard = 33,
}

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = 40;

lazy_static! {
    static ref PICS: Mutex<ChainedPics> =
        unsafe { Mutex::new(ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)) };
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[InterruptIndex::Timer as u8].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();

    unsafe {
        PICS.lock().initialize();
    }

    x86_64::instructions::interrupts::enable();
}

/* Breakpoint Handler
*/
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    if let Some(gm) = super::super::DISPLAY.lock().as_mut() {
        gm.write_text(
            super::super::PURPLE,
            "x86_64 breakpoint interrupt".as_bytes(),
        );
        gm.write_text(super::super::ORANGE, "\n> stack frame\n".as_bytes());
    }

    // EOI
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Breakpoint as u8);
    }
}

/* Double Fault Handler
Occurs when kernel ignores an interrupt
*/
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    if let Some(gm) = super::super::DISPLAY.lock().as_mut() {
        gm.write_text(super::super::ORANGE, "\n> stack frame".as_bytes());
    }
    panic!("double fault!");
    loop {}
}

/* Timer Handler
Periodic interupt to provide the time to the kernel */
extern "x86-interrupt" fn timer_handler(stack_frame: InterruptStackFrame) {
    // temp debugging
    //if let Some(gm) = super::super::DISPLAY.lock().as_mut() {
    //    gm.write_text(PURPLE, ".".as_bytes());
    //}

    // EOI
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

/* Keyboard Interupt */
extern "x86-interrupt" fn keyboard_handler(stack_frame: InterruptStackFrame) {
    let mut port = Port::<u8>::new(0x60); // PS/2 data port u8
    let scancode = unsafe { port.read() };

    // this is temporary
    //if let Some(gm) = super::super::DISPLAY.lock().as_mut() {
    //    gm.write_text(PURPLE, "scancode: ".as_bytes());
    //    gm.write_formated_text_u32(super::super::WHITE, "{} ".as_bytes(), scancode as u32);
    //}

    // register the scancode
    if let Some(km) = super::super::KEYBOARD.lock().as_mut() {
        km.register_scancode(scancode);
    }
    // EOI
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}
