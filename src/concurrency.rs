use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Локальный счётчик на каждый вызов — изоляция между тестами.
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::with_capacity(threads);

    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            for _ in 0..iterations {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    counter.load(Ordering::SeqCst)
}

/// Глобальный счётчик — отдельно.
static GLOBAL_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn read_current_counter() -> u64 {
    GLOBAL_COUNTER.load(Ordering::SeqCst)
}

pub fn reset_counter() {
    GLOBAL_COUNTER.store(0, Ordering::SeqCst);
}

pub fn increment_global_counter(value: u64) {
    GLOBAL_COUNTER.fetch_add(value, Ordering::SeqCst);
}