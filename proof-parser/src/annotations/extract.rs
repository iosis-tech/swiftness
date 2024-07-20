use super::annotation_kind::ZAlpha;
use alloc::format;
use alloc::vec::Vec;
use num_bigint::BigUint;
use regex::Regex;

pub trait FromStrHex: Sized {
    fn from_str_hex(val: &str) -> Option<Self>;
}

impl FromStrHex for BigUint {
    fn from_str_hex(mut val: &str) -> Option<Self> {
        val = val.trim();
        if val.starts_with("0x") {
            val = &val[2..];
        }
        BigUint::parse_bytes(val.as_bytes(), 16)
    }
}

pub fn extract_z_and_alpha(annotations: &[&str]) -> anyhow::Result<ZAlpha> {
    let re = Regex::new(
        r"V->P: /cpu air/STARK/Interaction: Interaction element #\d+: Field Element\(0x([0-9a-f]+)\)",
    ).unwrap();

    let mut interaction_elements = Vec::new();

    for annotation in annotations {
        for cap in re.captures_iter(annotation) {
            match BigUint::from_str_hex(&cap[1]) {
                Some(value) => interaction_elements.push(value),
                None => anyhow::bail!("Unable to parse"),
            }
        }
    }

    // Make sure the number of interaction_elements is as expected
    if ![3, 6].contains(&interaction_elements.len()) {
        anyhow::bail!("Unexpected number of interaction elements: {}", interaction_elements.len());
    }

    let z_alpha =
        ZAlpha { z: interaction_elements[0].clone(), alpha: interaction_elements[1].clone() };

    Ok(z_alpha)
}

pub fn extract_annotations(
    annotations: &[&str],
    prefix: &str,
    kind: &str,
) -> anyhow::Result<Vec<BigUint>> {
    let pattern = format!(r"P->V\[(\d+):(\d+)\]: /cpu air/{prefix}: .*{kind}\((.+)\)");
    let re = Regex::new(&pattern).unwrap();
    let mut res = Vec::new();

    for line in annotations {
        if let Some(cap) = re.captures(line) {
            let str_value = &cap[3];
            if kind == "Field Elements" {
                res.extend(str_value.split(',').filter_map(BigUint::from_str_hex));
            } else if let Some(val) = BigUint::from_str_hex(str_value) {
                res.push(val)
            }
        }
    }

    Ok(res)
}
