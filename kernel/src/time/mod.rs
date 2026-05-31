use core::hint::spin_loop;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;

pub const TICK_HZ: usize = 100; 


pub struct TimeManager {
    boot_time: usize,
    tick_counter: AtomicUsize,
}

impl TimeManager {
    pub fn new(boot_time:usize) -> Self {
        Self { 
            boot_time,
            tick_counter:AtomicUsize::new(0),
        }
    }

    pub fn register_tick(&self) {
        self.tick_counter.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_uptime_ms(&self) -> usize{
        let current_tick = self.tick_counter.load(Ordering::Relaxed);
        self.ticks_to_ms(current_tick)
    }

    pub fn time_between_ticks_ms(&self, start: usize, end: usize) -> usize {
        self.ticks_to_ms(end - start)
    }

    pub fn ms_to_ticks(&self, time:usize) -> usize {
        (time * TICK_HZ) / 1000
    }

    pub fn ticks_to_ms(&self, ticks:usize) -> usize {
        (ticks*1000) / TICK_HZ
    }
}

pub fn sleep(duration: usize) {
    let mut end = 0;
    if let Some(tm) = super::TIME.lock().as_mut() {
        let start = tm.tick_counter.load(Ordering::Relaxed);
        end = start + tm.ms_to_ticks(duration);
    }

    loop {
        let current = { super::TIME.lock().as_ref().unwrap().tick_counter.load(Ordering::Relaxed) };
        if current > end {
            break;
        }
        spin_loop(); // tells cpu to wait
    }
}