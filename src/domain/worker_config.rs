use std::fmt;
use std::num::NonZeroUsize;

/// Validated configuration containing a positive requested worker count.
///
/// Creating a configuration does not start workers or query CPU capacity.
/// The requested count is not capped by the detected parallelism.
#[derive(Debug, Clone, Copy)]
pub struct WorkerConfig {
    workers: NonZeroUsize,
}

/// An error encountered while validating a [`WorkerConfig`].
#[derive(Debug, Clone, Copy)]
pub enum WorkerConfigError {
    /// The requested worker count was zero; at least one worker is required.
    ZeroWorkers,
}

impl WorkerConfig {
    /// Creates a configuration with the requested worker count.
    ///
    /// # Errors
    ///
    /// Returns [`WorkerConfigError::ZeroWorkers`] when `workers` is zero.
    pub fn new(workers: usize) -> Result<Self, WorkerConfigError> {
        let config = NonZeroUsize::new(workers).ok_or(WorkerConfigError::ZeroWorkers)?;
        Ok(Self { workers: config })
    }
    /// Returns the validated, nonzero worker count.
    ///
    /// Use [`NonZeroUsize::get`] to obtain its value as a `usize`.
    pub fn workers(&self) -> NonZeroUsize {
        self.workers
    }
}

impl fmt::Display for WorkerConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkerConfigError::ZeroWorkers => write!(f, "worker count must be at least 1"),
        }
    }
}

impl std::error::Error for WorkerConfigError {}
