#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

macro_rules! import_module_if_layout {
    ($mod_name:ident) => {
        #[cfg(any(
            feature = "dex",
            feature = "recursive",
            feature = "recursive_with_poseidon",
            feature = "small",
            feature = "starknet",
            feature = "starknet_with_keccak",
            feature = "dynamic"
        ))]
        pub mod $mod_name;
    };
}

import_module_if_layout!(commit);
import_module_if_layout!(oods);
import_module_if_layout!(stark);
import_module_if_layout!(verify);

pub mod config;
pub mod queries;
pub mod types;

#[cfg(test)]
pub mod fixtures;
#[cfg(test)]
pub mod tests;
