pub mod intel_x86_64;

/* Interupt Descriptor Table */
pub fn init_idt() {
    intel_x86_64::init_idt();
}

/* programmable interval timer */
pub fn init_pit() {
    intel_x86_64::init_pit();
}
