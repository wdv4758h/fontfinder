use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::thread;
use std::time::Duration;

pub static RUN_FC_CACHE: AtomicBool = ATOMIC_BOOL_INIT;

pub fn fc_cache_event_loop() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(100));
            if RUN_FC_CACHE.swap(false, Ordering::Relaxed) {
                let _ = Command::new("fc-cache")
                    .arg("-f")
                    .spawn()
                    .map(|mut child| child.wait());
            }
        }
    });
}
