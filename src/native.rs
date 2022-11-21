use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn now() -> i32 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("must get duration");
    let ms = duration.as_millis();
    ms as i32
}

pub async fn async_delay(timeout: i32) {
    let duration = Duration::from_millis(timeout as u64);
    thread::sleep(duration);
}
