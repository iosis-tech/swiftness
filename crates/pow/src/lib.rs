#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod config;
pub mod pow;

#[cfg(any(test, feature = "test_fixtures"))]
pub mod fixtures;
#[cfg(test)]
pub mod tests;
