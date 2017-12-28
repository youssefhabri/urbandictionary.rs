//! Bridged support between different HTTP clients.

#[cfg(feature = "hyper-support")]
pub mod hyper;
#[cfg(feature = "reqwest-support")]
pub mod reqwest;
