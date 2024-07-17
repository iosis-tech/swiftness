use std::convert::TryFrom;

use crate::{json_parser::ProofJSON, stark_proof::StarkProof};

mod annotations;
mod ast;
mod builtins;
mod json_parser;
mod layout;
mod stark_proof;
mod utils;

extern crate clap;
extern crate num_bigint;
extern crate regex;
extern crate serde;

pub use ast::{Expr, Exprs};

pub struct ParseStarkProof {
    pub config: Exprs,
    pub public_input: Exprs,
    pub unsent_commitment: Exprs,
    pub witness: Exprs,
}

pub fn parse(input: String) -> anyhow::Result<ParseStarkProof> {
    let proof_json = serde_json::from_str::<ProofJSON>(&input)?;
    let stark_proof = StarkProof::try_from(proof_json)?;
    Ok(ParseStarkProof {
        config: Exprs::from(stark_proof.config),
        public_input: Exprs::from(stark_proof.public_input),
        unsent_commitment: Exprs::from(stark_proof.unsent_commitment),
        witness: Exprs::from(stark_proof.witness),
    })
}
