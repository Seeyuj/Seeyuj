//! # sy_core (NIV 2)
//!
//! Pure simulation logic - the sanctuary.
//! This crate contains deterministic game logic with no I/O.
//!
//! ## Rules
//! - No async runtime
//! - No filesystem or network access
//! - No randomness from std (use injected IRng)
//! - No time from std (use injected ISimClock)

pub mod ports;
pub mod sim;
pub mod world;
