use bootloader_api::info::{MemoryRegionKind, MemoryRegions};

use crate::{println};

pub struct MemoryManager {
    usable_pages: u64,
}

impl MemoryManager {
    
}

#[cfg(debug_assertions)]
pub fn debug_memory_map (mem_map: &MemoryRegions){
    println!();
    println!("--- Memory Map ---");
    for (i, region) in mem_map.iter().enumerate() {
        println!("{},size ({:.2}) GiB, {:?}", i,(region.end-region.start) as f32 / (1024 as u64).pow(3) as f32, region);
    }
    println!("------------------");
    println!();
}

pub fn calculate_free_pages (mem_map: &MemoryRegions, page_size: u64) -> u64{
    let mut sum_free_pages = 0;
    for (i, region) in mem_map.iter().enumerate() {
        if region.kind == MemoryRegionKind::Usable {
            sum_free_pages += (region.end - region.start) / page_size
        }
    }
    println!("> {} free pages", sum_free_pages);
    println!("> {:.2} GiB memory", (sum_free_pages * page_size) as f32 / (1024 as u64).pow(3) as f32);
    sum_free_pages
}