use num_bigint::BigUint;
use serde::Deserialize;
use std::{collections::BTreeMap, convert::TryInto, fmt::Display};

// For now only the recursive and starknet layouts is supported
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Dex,
    Plain,
    Recursive,
    RecursiveWithPoseidon,
    Small,
    Starknet,
    StarknetWithKeccak,
}

impl Layout {
    pub fn get_consts(&self) -> LayoutConstants {
        match self {
            Layout::Dex => LayoutConstants::dex(),
            Layout::Plain => LayoutConstants::plain(),
            Layout::Recursive => LayoutConstants::recursive(),
            Layout::RecursiveWithPoseidon => LayoutConstants::recursive_with_poseidon(),
            Layout::Small => LayoutConstants::small(),
            Layout::Starknet => LayoutConstants::starknet(),
            Layout::StarknetWithKeccak => LayoutConstants::starknet_with_keccak(),
        }
    }
    pub fn get_dynamics_or_consts(
        &self,
        dynamic_params: &Option<BTreeMap<String, BigUint>>,
    ) -> Option<LayoutConstants> {
        let consts = self.get_consts();

        let dynamic_params = match dynamic_params {
            Some(dp) => dp,
            None => return Some(consts),
        };

        Some(LayoutConstants {
            cpu_component_step: dynamic_params
                .get("cpu_component_step")
                .map(<&BigUint>::try_into)
                .map(Result::ok)?
                .unwrap_or(consts.cpu_component_step),
            constraint_degree: dynamic_params
                .get("constraint_degree")
                .map(<&BigUint>::try_into)
                .map(Result::ok)?
                .unwrap_or(consts.constraint_degree),
            num_columns_first: dynamic_params
                .get("num_columns_first")
                .map(<&BigUint>::try_into)
                .map(Result::ok)?
                .unwrap_or(consts.num_columns_first),
            num_columns_second: dynamic_params
                .get("num_columns_second")
                .map(<&BigUint>::try_into)
                .map(Result::ok)?
                .unwrap_or(consts.num_columns_second),
        })
    }
    pub fn bytes_encode(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layout::Dex => write!(f, "dex"),
            Layout::Plain => write!(f, "plain"),
            Layout::Recursive => write!(f, "recursive"),
            Layout::RecursiveWithPoseidon => write!(f, "recursive_with_poseidon"),
            Layout::Small => write!(f, "small"),
            Layout::Starknet => write!(f, "starknet"),
            Layout::StarknetWithKeccak => write!(f, "starknet_with_keccak"),
        }
    }
}

pub(crate) struct LayoutConstants {
    pub cpu_component_step: u32,
    pub constraint_degree: u32,
    pub num_columns_first: u32,
    pub num_columns_second: u32,
}

impl LayoutConstants {
    pub fn recursive() -> Self {
        LayoutConstants {
            constraint_degree: 2,
            cpu_component_step: 1,
            num_columns_first: 7,
            num_columns_second: 3,
        }
    }
    pub fn starknet() -> Self {
        LayoutConstants {
            constraint_degree: 2,
            cpu_component_step: 1,
            num_columns_first: 9,
            num_columns_second: 1,
        }
    }
    pub fn small() -> Self {
        LayoutConstants {
            constraint_degree: 2,
            cpu_component_step: 1,
            num_columns_first: 23,
            num_columns_second: 2,
        }
    }
    pub fn recursive_with_poseidon() -> Self {
        LayoutConstants {
            constraint_degree: 2,
            cpu_component_step: 1,
            num_columns_first: 6,
            num_columns_second: 2,
        }
    }
    pub fn plain() -> Self {
        LayoutConstants {
            constraint_degree: 2,
            cpu_component_step: 1,
            num_columns_first: 6,
            num_columns_second: 2,
        }
    }
    pub fn starknet_with_keccak() -> Self {
        LayoutConstants {
            constraint_degree: 2,
            cpu_component_step: 1,
            num_columns_first: 12,
            num_columns_second: 3,
        }
    }
    pub fn dex() -> Self {
        LayoutConstants {
            constraint_degree: 2,
            cpu_component_step: 1,
            num_columns_first: 21,
            num_columns_second: 1,
        }
    }
}
