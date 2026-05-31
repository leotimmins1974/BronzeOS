pub mod intel_x86_64;

pub fn init() {
    intel_x86_64::init_idt(); // Temporararily assuming x86
}
