use crate::json_parser::MemorySegmentAddress;
use std::collections::BTreeMap;

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
            "range_check96" => Some(Builtin::RangeCheck96),
            "add_mod" => Some(Builtin::AddMod),
            "mul_mod" => Some(Builtin::MulMod),
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
            .filter_map(|(k, v)| {
                let builtin = Builtin::from_str(&k)?;
                Some((builtin, v))
            })
            .collect::<Vec<_>>();
        segments.sort_by_key(|(builtin, _)| Builtin::ordered().iter().position(|b| b == builtin));
        segments.into_iter().map(|(_, segment)| segment).collect()
    }
}
