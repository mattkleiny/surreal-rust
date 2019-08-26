//! A job system for parallel processing.

/// A job that can be executed in the job manager.
pub trait Job {
  fn execute(&mut self);
}

/// Parallel job manager.
pub struct JobManager {}

impl JobManager {
  pub fn new() -> Self {
    Self {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestJob {
    iterations: usize
  }

  impl Job for TestJob {
    fn execute(&mut self) {
      for i in 0..self.iterations {
        println!("Step {}", i);
      }
    }
  }

  #[test]
  fn it_should_execute_basic_jobs() {
    unimplemented!()
  }
}