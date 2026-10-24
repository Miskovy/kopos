//! Host-independent domain rules and values.
//!
//! Future types own and protect their invariants through private fields and
//! validated construction. Keep this module independent of application policy,
//! adapters, bindings, and JavaScript engine handles.
pub(crate) mod box_filter;
pub(crate) mod image_input;

pub(crate) mod worker_config;
