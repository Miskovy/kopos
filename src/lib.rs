//! Kopos research foundation for worker execution and parallel computation.
//!
//! The crate currently establishes module boundaries only. Runtime behavior,
//! native bindings, and execution backends belong to later reviewed stages.
//!
//! Internal dependencies point toward domain rules and focused ports. Concrete
//! host facilities belong in adapters; the public API hides those details.

#![warn(missing_docs)]

mod adapters;
pub mod api;
mod application;
mod domain;
mod ports;
// pub mod common;
