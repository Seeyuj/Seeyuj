//! # sy_infra (NIV 3)
//!
//! Real I/O implementations for the core's port interfaces.
//! This is the only place where sy_protocol <-> sy_api mapping happens.

pub mod clock;
pub mod net;
pub mod observability;
pub mod rng;
pub mod store;
