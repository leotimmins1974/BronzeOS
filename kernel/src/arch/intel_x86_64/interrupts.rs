use pic8259::ChainedPics;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::println;

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
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[InterruptIndex::Timer as u8].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard_handler);
        idt
    };
}

pub fn init() {
    /* Enable Interupts */
    IDT.load();
    unsafe {
        PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();

    /* Configure PIT */
    init_pit();
}

fn init_pit() {
    let mut cmd = Port::<u8>::new(0x43);
    let mut data = Port::<u8>::new(0x40);

    let div = (1193182 / crate::time::TICK_HZ) as u16;

    // pid programming, chanel 0, low then high byte, square wave, binary counter
    unsafe {
        cmd.write(0b00110110);
        data.write((div & 0xff) as u8); //left 8 bits
        data.write((div >> 8) as u8); //right 8 bits
    }
}

/* Breakpoint Handler */
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("breakpoint interrupt");

    // EOI
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Breakpoint as u8);
    }
}

/* Double Fault Handler */
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("double fault!");
}

/* Timer Handler */
extern "x86-interrupt" fn timer_handler(stack_frame: InterruptStackFrame) {
    // register tick
    if let Some(tm) = crate::TIME.lock().as_mut() {
        tm.register_tick();
    }

    // EOI
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

/* Keyboard Interupt */
extern "x86-interrupt" fn keyboard_handler(stack_frame: InterruptStackFrame) {
    // read scancode
    let mut port = Port::<u8>::new(0x60); // PS/2 data port u8
    let scancode = unsafe { port.read() };

    // register the scancode
    if let Some(km) = crate::KEYBOARD.lock().as_mut() {
        km.register_scancode(scancode);
    }
    // EOI
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}
