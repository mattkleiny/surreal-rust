//! Diagnostics and monitoring for the ECS.

pub use debugging::*;
pub use metrics::*;
pub use profiler::*;

/// A collection of diagnostics and monitoring tools.
#[derive(Default)]
pub struct WorldDiagnostics {
  debuggers: Vec<Box<dyn DebugListener>>,
  profilers: Vec<Box<dyn ProfileListener>>,
  collectors: Vec<Box<dyn MetricsListener>>,
}

impl DebugListener for WorldDiagnostics {
  fn on_debug_event(&mut self, event: &DebuggingEvent) {
    for debugger in &mut self.debuggers {
      debugger.on_debug_event(event);
    }
  }
}

impl MetricsListener for WorldDiagnostics {
  fn on_metric_event(&mut self, event: &MetricsEvent) {
    for collector in &mut self.collectors {
      collector.on_metric_event(event);
    }
  }
}

impl ProfileListener for WorldDiagnostics {
  fn on_profiling_event(&mut self, event: &ProfilingEvent) {
    for profiler in &mut self.profilers {
      profiler.on_profiling_event(event);
    }
  }
}

mod debugging {
  pub enum DebuggingEvent {}

  pub trait DebugListener {
    fn on_debug_event(&mut self, event: &DebuggingEvent);
  }
}

mod profiler {
  pub enum ProfilingEvent {}

  pub trait ProfileListener {
    fn on_profiling_event(&mut self, event: &ProfilingEvent);
  }
}

mod metrics {
  pub enum MetricsEvent {}

  pub trait MetricsListener {
    fn on_metric_event(&mut self, event: &MetricsEvent);
  }
}
