#![no_std]

use funvec::{FunVec, FUNVEC_QUERIES};
use starknet_crypto::Felt;
use vector::types::{Query, QueryWithDepth};

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod table;
pub mod vector;

#[derive(Debug, Clone, Copy, Default)]
pub struct CacheCommitment {
    pub vector_queries: FunVec<Query, FUNVEC_QUERIES>, // Initialized based on values.
    // Tree structure with queries with original queries on the left.
    pub shifted_queries: FunVec<QueryWithDepth, { FUNVEC_QUERIES * 3 }>, // Uninitialized.
    pub points: FunVec<Felt, 2048>,                                      // Uninitialized.
}

unsafe impl bytemuck::Pod for CacheCommitment {}
unsafe impl bytemuck::Zeroable for CacheCommitment {}
