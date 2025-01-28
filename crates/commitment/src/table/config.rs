use crate::vector;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_crypto::Felt;

#[serde_as]
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[cfg_attr(
        feature = "std",
        serde_as(as = "starknet_core::serde::unsigned_field_element::UfeHex")
    )]
    pub n_columns: Felt,
    pub vector: vector::config::Config,
}
