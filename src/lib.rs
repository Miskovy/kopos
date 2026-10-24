//! Kopos research foundation for worker execution and parallel computation.
//!
//! The crate provides build-target platform information, runtime parallel-capacity
//! discovery, validated worker configuration, and a serial grayscale box-filter
//! experiment through [`api::filter_grayscale`].
//! Worker runtime behavior, native bindings, and execution backends belong to
//! later reviewed stages.
//!
//! Internal dependencies point toward domain rules and focused ports. Concrete
//! host facilities belong in adapters; the public API hides those details.

#![warn(missing_docs)]

mod adapters;
pub mod api;
mod application;
mod domain;
mod ports;

mod common;
