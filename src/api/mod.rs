//! Public entry points and types for Kopos consumers.
//!
//! [`PlatformInfo`] describes the compilation target, [`CpuInfo`] reports
//! estimated parallel capacity, and [`WorkerConfig`] validates a requested
//! worker count. Invalid configuration produces [`WorkerConfigError`].
//! Implementation modules remain private; worker execution APIs are future work.

pub use crate::common::structs::{CpuInfo, PlatformInfo};
pub use crate::domain::worker_config::{WorkerConfig, WorkerConfigError};
