#![no_std]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod table;
pub mod vector;
