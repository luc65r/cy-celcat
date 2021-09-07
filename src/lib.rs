#![doc = include_str!("../README.md")]

pub mod entities;
#[cfg(feature = "fetch")]
pub mod fetch;
pub mod fetchable;

pub use entities::*;
#[cfg(feature = "fetch")]
pub use fetch::Celcat;
