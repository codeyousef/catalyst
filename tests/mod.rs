/// Catalyst IDE Test Suite
/// 
/// This module contains all tests for the Catalyst IDE project.
/// Following strict Test-Driven Development (TDD) principles.

pub mod performance;
pub mod integration;
pub mod unit;

#[cfg(test)]
mod test_utils {
    use std::time::{Duration, Instant};
    use std::process::{Command, Stdio};

    /// Helper function to measure execution time
    pub fn measure_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Helper function to spawn Catalyst process for testing
    pub fn spawn_catalyst_process() -> Result<std::process::Child, std::io::Error> {
        Command::new("cargo")
            .args(&["run", "--bin", "catalyst"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    }
}