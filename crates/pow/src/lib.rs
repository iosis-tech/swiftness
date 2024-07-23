#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod config;
pub mod fixtures;
pub mod pow;

#[cfg(test)]
pub mod tests;
