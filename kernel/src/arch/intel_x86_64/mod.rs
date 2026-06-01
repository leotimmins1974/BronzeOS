mod interrupts;
pub mod memory;

pub fn init() {
    interrupts::init();
}
