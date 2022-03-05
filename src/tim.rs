use std::time::{Duration, Instant};

pub struct ScopedTimer {
    start: Instant,
    msg: String,
}

impl ScopedTimer {
    pub fn new(msg: String) -> Self {
        Self {
            start: Instant::now(),
            msg,
        }
    }
    pub fn so_far (&self) -> Duration {
        self.start.elapsed()
    }
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        eprintln!("{} took: {:?}", self.msg, self.start.elapsed())
    }
}
