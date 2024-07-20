#![no_std]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod commit;
pub mod config;
pub mod fixtures;
pub mod oods;
pub mod queries;
pub mod stark;
pub mod types;
pub mod verify;

#[cfg(test)]
pub mod tests;
