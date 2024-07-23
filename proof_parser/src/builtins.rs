use std::collections::HashMap;

use crate::json_parser::MemorySegmentAddress;

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
}

impl Builtin {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "program" => Some(Builtin::Program),
            "execution" => Some(Builtin::Execution),
            "output" => Some(Builtin::Output),
            "pedersen" => Some(Builtin::Pedersen),
            "range_check" => Some(Builtin::RangeCheck),
            "ecdsa" => Some(Builtin::Ecdsa),
            "bitwise" => Some(Builtin::Bitwise),
            "ec_op" => Some(Builtin::EcOp),
            "keccak" => Some(Builtin::Keccak),
            "poseidon" => Some(Builtin::Poseidon),
            _ => None,
        }
    }
    pub fn ordered() -> Vec<Self> {
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
        ]
    }
    pub fn sort_segments(
        memory_segments: HashMap<String, MemorySegmentAddress>,
    ) -> Vec<MemorySegmentAddress> {
        let mut segments = memory_segments
            .into_iter()
            .filter_map(|(k, v)| {
                let builtin = Builtin::from_str(&k)?;
                Some((builtin, v))
            })
            .collect::<Vec<_>>();
        segments.sort_by_key(|(builtin, _)| Builtin::ordered().iter().position(|b| b == builtin));
        segments.into_iter().map(|(_, segment)| segment).collect()
    }
}
