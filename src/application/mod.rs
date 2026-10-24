//! Coordination of domain operations and their ownership transitions.
//!
//! Future use cases depend on domain values and focused ports. Receive concrete
//! dependencies through construction rather than discovering global services.
//! Keep host loading and foreign value conversion inside adapters.

pub(crate) mod filter;
