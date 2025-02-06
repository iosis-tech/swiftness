#![no_std]

use funvec::{FunVec, FUNVEC_QUERIES};
use vector::types::Query;

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod table;
pub mod vector;

#[derive(Debug, Clone, Copy, Default)]
pub struct CacheCommitment {
    pub vector_queries: FunVec<Query, FUNVEC_QUERIES>,
}

unsafe impl bytemuck::Pod for CacheCommitment {}
unsafe impl bytemuck::Zeroable for CacheCommitment {}
