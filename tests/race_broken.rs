//! Тест специально содержит гонку — для TSan before-лога.
//! Запуск: ... broken_race_increment -- --ignored --nocapture

use std::thread;

static mut BROKEN_COUNTER: u64 = 0;

#[test]
#[ignore]
fn broken_race_increment() {
    unsafe { BROKEN_COUNTER = 0; }

    let mut handles = Vec::new();
    for _ in 0..4 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    BROKEN_COUNTER += 1; // DATA RACE
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    let result = unsafe { BROKEN_COUNTER };
    println!("Broken counter result: {} (expected 4000)", result);
    assert!(result <= 4000);
}