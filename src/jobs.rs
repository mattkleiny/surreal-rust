//! A job system for parallel processing.


/// A job that can be executed in the job manager.
pub trait Job {
  fn execute(&mut self);
}

/// Handle for the execution of some job, not unlike a future.
pub struct JobHandle(usize);

/// Parallel job manager.
pub struct JobManager {}

impl JobManager {
  pub const fn new() -> Self {
    Self {}
  }

  /// Schedules the given job for execution, taking ownership of it.
  pub fn schedule<J : Job>(&mut self, mut job: J) -> JobHandle {
    // TODO: implement me
    job.execute();
    JobHandle(1024)
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
    let mut manager = JobManager::new();

    manager.schedule(TestJob {
      iterations: 100
    });
  }
}