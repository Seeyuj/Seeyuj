//! # Ports
//!
//! Interfaces (traits) defining the needs of the core.
//! These are implemented by sy_infra or sy_testkit.

pub mod event_log;
pub mod hasher;
pub mod rng;
pub mod sim_clock;
pub mod store;
