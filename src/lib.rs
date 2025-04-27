use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct ScopeTimer {
    now: Instant,
}

impl ScopeTimer {
    pub fn new() -> Self {
        Self {
            now: Instant::now(),
        }
    }

    pub fn elapsed_time(&self) -> Duration {
        Instant::now().duration_since(self.now)
    }

    pub fn report_elapsed_time(&self) {
        println!("Time elapsed: {:?}", self.elapsed_time());
    }
}

impl Drop for ScopeTimer {
    fn drop(&mut self) {
        self.report_elapsed_time();
    }
}
