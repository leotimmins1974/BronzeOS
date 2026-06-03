use bootloader_api::info::MemoryRegions;

use crate::memory;

const PAGE_SIZE: u64 = 4096;

pub fn init(mem_map: &'static MemoryRegions, phys_mem_offset: u64) {
    memory::init(mem_map, phys_mem_offset, PAGE_SIZE);
}
