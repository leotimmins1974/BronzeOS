use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::paging::{Page, page};

use crate::println;

struct MemoryManager {
    page_size: u64,
    phys_mem_offset: u64,
    mem_map: Option<&'static MemoryRegions>,
}

lazy_static! {
    static ref MEMORY: Mutex<MemoryManager> = Mutex::new(MemoryManager {
        page_size: 0,
        phys_mem_offset: 0,
        mem_map: None
    });
}

#[cfg(debug_assertions)]
fn debug_memory_map(mem_map: &MemoryRegions) {
    println!("--- Memory Map ---");
    for (i, region) in mem_map.iter().enumerate() {
        println!(
            "{},size ({:.2}) GiB, {:?}",
            i,
            (region.end - region.start) as f32 / (1024 as u64).pow(3) as f32,
            region
        );
    }
    println!("------------------");
    println!();
}

fn calculate_free_pages(mem_map: &MemoryRegions, page_size: u64) {
    let mut sum_free_pages = 0;
    for (i, region) in mem_map.iter().enumerate() {
        if region.kind == MemoryRegionKind::Usable {
            sum_free_pages += (region.end - region.start) / page_size
        }
    }
    println!("> {} free pages", sum_free_pages);
    println!(
        "> {:.2} GiB memory",
        (sum_free_pages * page_size) as f32 / (1024 as u64).pow(3) as f32
    );
}

#[repr(C)]
struct PageNode {
    next: *mut PageNode,
}

pub struct PMMListEntry {
    head: Option<*mut PageNode>,
    free_pages: usize,
}

unsafe impl Send for PMMListEntry {}

lazy_static! {
    static ref PMM: Mutex<PMMListEntry> = Mutex::new(PMMListEntry {
        head: Some(core::ptr::null_mut()),
        free_pages: 0
    });
}

pub fn init(mem_map: &'static MemoryRegions, phys_mem_offset: u64, page_size: u64) {
    MEMORY.lock().phys_mem_offset = phys_mem_offset;
    MEMORY.lock().page_size = page_size;
    MEMORY.lock().mem_map = Some(mem_map);

    #[cfg(debug_assertions)]
    debug_memory_map(mem_map);

    calculate_free_pages(mem_map, page_size);

    /* create virtual memory linked list */
    let mut head: *mut PageNode = core::ptr::null_mut();
    let mut count = 0;

    for region in mem_map.iter() {
        if region.kind == MemoryRegionKind::Usable {
            let mut current_page = region.start;

            while current_page + page_size <= region.end {
                let virt_adress = current_page + phys_mem_offset;
                let node_ptr = virt_adress as *mut PageNode;

                unsafe { (*node_ptr).next = head }

                head = node_ptr;

                count += 1;
                current_page += page_size;
            }
        }
    }

    PMM.lock().head = Some(head);
    PMM.lock().free_pages = count;
}

pub unsafe fn alloc_page() -> Option<u64> {
    // no free pages
    if PMM.lock().free_pages == 0 {
        return None;
    }

    // get current and new head
    let mut head = PMM.lock().head.expect("No head in alloc_page");
    let mut new_head = unsafe { head.read().next };

    // assign new head to PMM struct
    PMM.lock().head = Some(new_head);

    // assign the head
    return Some(head.addr() as u64);
}

pub unsafe fn free_page(adress: u64) {
    assert!(adress % MEMORY.lock().page_size == 0)
}
