#![no_std]

use funvec::{FunVec, FUNVEC_QUERIES};
use layer::FriLayerQuery;

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod config;
pub mod first_layer;
pub mod formula;
pub mod fri;
pub mod group;
pub mod last_layer;
pub mod layer;
pub mod types;

#[cfg(any(test, feature = "test_fixtures"))]
pub mod fixtures;
#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone, Copy, Default)]
pub struct FriVerifyCache {
    pub fri_queries: FunVec<FriLayerQuery, FUNVEC_QUERIES>,
}

unsafe impl bytemuck::Pod for FriVerifyCache {}
unsafe impl bytemuck::Zeroable for FriVerifyCache {}
