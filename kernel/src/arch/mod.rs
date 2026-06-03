use bootloader_api::info::MemoryRegions;

#[cfg(target_arch = "x86_64")]
mod intel_x86_64;

#[cfg(target_arch = "aarch64")]
mod aarch64;

pub fn init(mem_map: &'static MemoryRegions, phys_mem_offset: u64) {
    #[cfg(target_arch = "x86_64")]
    {
        /* intel_x86_64 initiation pathway */
        intel_x86_64::init();
        intel_x86_64::memory::init(mem_map, phys_mem_offset);
    }

    #[cfg(target_arch = "aarch64")]
    {
        /* aarm64 initiation pathway */
        compile_error!("aarm64 is not currently supported");
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        compile_error!("architecture is not and will not be supported")
    }
}
