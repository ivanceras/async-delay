use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod wasm;

/// return the epoc time in milliseconds when used in native platform
/// return the time origin when used in the browser
pub fn now() -> i32 {
    #[cfg(not(target_arch = "wasm32"))]
    let t1 = native::now();
    #[cfg(target_arch = "wasm32")]
    let t1 = wasm::now();
    t1
}

pub async fn delay(timeout: i32) {
    println!("delaying {}ms", timeout);
    #[cfg(not(target_arch = "wasm32"))]
    native::async_delay(timeout).await;
    #[cfg(target_arch = "wasm32")]
    wasm::async_delay(timeout).await;
}

pub struct Throttle {
    last_exec: AtomicI32,
    dirty: AtomicBool,
    /// interval in ms
    interval: i32,
    is_executing: AtomicBool,
}

impl Throttle {
    pub fn from_interval(interval: i32) -> Self {
        Self {
            last_exec: AtomicI32::new(0),
            dirty: AtomicBool::new(false),
            interval,
            is_executing: AtomicBool::new(false),
        }
    }

    pub fn set_executing(&self, is_executing: bool) {
        self.is_executing.store(is_executing, Ordering::Relaxed);
    }

    fn set_dirty(&self, is_dirty: bool) {
        self.dirty.store(is_dirty, Ordering::Relaxed);
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Relaxed)
    }

    fn is_executing(&self) -> bool {
        self.is_executing.load(Ordering::Relaxed)
    }

    fn last_exec(&self) -> i32 {
        self.last_exec.load(Ordering::Relaxed)
    }

    pub fn should_execute(&self) -> bool {
        if self.is_executing() {
            false
        } else {
            let elapsed = now() - self.last_exec();
            if elapsed >= self.interval {
                true
            } else {
                //log::info!("will NOT execute but marked as dirty");
                self.set_dirty(true);
                false
            }
        }
    }

    /// calculate the time duration needed to be able execute the function from now.
    pub fn remaining_time(&self) -> Option<i32> {
        let since = now() - self.last_exec();
        let rem = self.interval - (since % self.interval);
        if rem >= 0 {
            Some(rem)
        } else {
            None
        }
    }

    pub fn executed(&self) {
        self.last_exec.store(now(), Ordering::Relaxed);
        self.set_dirty(false);
        self.set_executing(false);
    }

    pub async fn call<F, R>(&self, f: F) -> Option<R::Output>
    where
        F: Fn() -> R,
        R: Future,
    {
        println!("here... {}", now());
        if self.should_execute() {
            println!("executing..");
            self.set_executing(true);
            let ret = f().await;
            self.executed();
            Some(ret)
        } else if self.is_dirty() {
            println!("in dirty..");
            let remaining = self.remaining_time().expect("must have a remaining time");
            delay(remaining + 1).await;
            self.set_executing(true);
            let ret = f().await;
            self.executed();
            Some(ret)
        } else {
            log::info!("not executing...");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::ready;

    #[test]
    fn it_works() {
        let time1 = now();
        let time2 = now();
        assert_eq!(time1, time2);
    }

    #[tokio::test]
    async fn throttles_call() {
        let throttle = Throttle::from_interval(100);
        println!("calling throttled..");
        let ret = throttle
            .call(|| {
                println!("hello world!");
                ready(42)
            })
            .await;
        println!("ret: {}", ret);
    }
}
