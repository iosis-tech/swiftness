use crate::json_parser::MemorySegmentAddress;
use std::{collections::BTreeMap, io, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Builtin {
    Program,
    Execution,
    Output,
    Pedersen,
    RangeCheck,
    Ecdsa,
    Bitwise,
    EcOp,
    Keccak,
    Poseidon,
    RangeCheck96,
    AddMod,
    MulMod,
}

impl Builtin {
    fn ordered() -> Vec<Self> {
        vec![
            Builtin::Program,
            Builtin::Execution,
            Builtin::Output,
            Builtin::Pedersen,
            Builtin::RangeCheck,
            Builtin::Ecdsa,
            Builtin::Bitwise,
            Builtin::EcOp,
            Builtin::Keccak,
            Builtin::Poseidon,
            Builtin::RangeCheck96,
            Builtin::AddMod,
            Builtin::MulMod,
        ]
    }
    pub fn sort_segments(
        memory_segments: BTreeMap<String, MemorySegmentAddress>,
    ) -> Vec<MemorySegmentAddress> {
        let mut segments = memory_segments
            .into_iter()
            .map(|(k, v)| (Builtin::from_str(&k).unwrap(), v))
            .collect::<Vec<_>>();
        segments.sort_by_key(|(builtin, _)| Builtin::ordered().iter().position(|b| b == builtin));
        segments.into_iter().map(|(_, segment)| segment).collect()
    }
}

impl FromStr for Builtin {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "program" => Ok(Builtin::Program),
            "execution" => Ok(Builtin::Execution),
            "output" => Ok(Builtin::Output),
            "pedersen" => Ok(Builtin::Pedersen),
            "range_check" => Ok(Builtin::RangeCheck),
            "ecdsa" => Ok(Builtin::Ecdsa),
            "bitwise" => Ok(Builtin::Bitwise),
            "ec_op" => Ok(Builtin::EcOp),
            "keccak" => Ok(Builtin::Keccak),
            "poseidon" => Ok(Builtin::Poseidon),
            "range_check96" => Ok(Builtin::RangeCheck96),
            "add_mod" => Ok(Builtin::AddMod),
            "mul_mod" => Ok(Builtin::MulMod),
            _ => Err(io::Error::new(io::ErrorKind::NotFound, "Builtin name not matched")),
        }
    }
}
