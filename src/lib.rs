use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct ScopeTimer {
    now: Instant,
    report: bool,
}

impl Default for ScopeTimer {
    fn default() -> Self {
        Self {
            now: Instant::now(),
            report: true,
        }
    }
}

impl ScopeTimer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_reporting(mut self, report: bool) -> Self {
        self.report = report;
        self
    }

    pub fn elapsed_time(&self) -> Duration {
        Instant::now().duration_since(self.now)
    }

    pub fn report_elapsed_time(&self) {
        if !self.report {
            return;
        }

        println!("Time elapsed: {:?}", self.elapsed_time());
    }
}

impl Drop for ScopeTimer {
    fn drop(&mut self) {
        self.report_elapsed_time();
    }
}
