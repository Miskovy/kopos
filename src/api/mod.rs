//! Public entry points and types for Kopos consumers.
//!
//! [`PlatformInfo`] describes the compilation target, [`CpuInfo`] reports
//! estimated parallel capacity, and [`WorkerConfig`] validates a requested
//! worker count. Invalid configuration produces [`WorkerConfigError`].
//!
//! [`filter_grayscale`] applies a synchronous, serial 5×5 box blur to an already
//! grayscale image. Invalid image input produces [`ImageInputError`]. The filter
//! does not use [`WorkerConfig`] or create execution workers.
//!
//! Implementation modules remain private; worker execution APIs are future work.

pub use crate::application::filter::filter_grayscale;
pub use crate::common::structs::{CpuInfo, PlatformInfo};
pub use crate::domain::image_input::ImageInputError;
pub use crate::domain::worker_config::{WorkerConfig, WorkerConfigError};
