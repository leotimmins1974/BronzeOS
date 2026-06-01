use bootloader_api::info::MemoryRegions;

use crate::memory;

const PAGE_SIZE: u64 = 4096;

pub fn init(mem_map: &MemoryRegions) {
    #[cfg(debug_assertions)]
    memory::debug_memory_map(mem_map);
    
    let free_pages = memory::calculate_free_pages(mem_map, PAGE_SIZE);
}