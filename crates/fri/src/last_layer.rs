use alloc::vec::Vec;
use starknet_core::types::NonZeroFelt;
use starknet_crypto::Felt;

use crate::layer::FriLayerQuery;

// Verifies FRI last layer by evaluating the given polynomial on the given points
// (=inverses of x_inv_values), and comparing the results to the given values.
pub fn verify_last_layer(quries: &[FriLayerQuery], coefficients: &[Felt]) -> Result<(), Error> {
    for query in quries {
        let horner_eval_result = horner_eval(
            &coefficients,
            Felt::ONE.field_div(&NonZeroFelt::from_felt_unchecked(query.x_inv_value)),
        );
        if horner_eval_result != query.y_value {
            return Err(Error::QueryMismatch { expected: query.y_value, got: horner_eval_result });
        }
    }

    Ok(())
}

// `horner_eval` is a function that evaluates a polynomial at a given point using Horner's method.
// `coefs` is an array of coefficients representing the polynomial in the format a0, a1, a2, ... an.
// `point` is the value at which the polynomial will be evaluated.
// The function returns the polynomial evaluation as `felt252`.
fn horner_eval(coefs: &[Felt], point: Felt) -> Felt {
    let mut result = Felt::from(0);
    for coef in coefs.iter().rev() {
        result = result * point + coef;
    }
    result
}

use thiserror_no_std::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Query mismatch: expected {expected}, got {got}")]
    QueryMismatch { expected: Felt, got: Felt },
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn test_horner_eval_0() {
        let coefs = Vec::new();
        let eval = horner_eval(&coefs, Felt::from(1));
        assert_eq!(eval, Felt::from(0));
    }

    #[test]
    fn test_horner_eval_1() {
        let coefs = vec![Felt::from(1)];
        let eval = horner_eval(&coefs, Felt::from(7));
        assert_eq!(eval, Felt::from(1));
    }

    #[test]
    fn test_horner_eval_2() {
        let coefs =
            vec![Felt::from(4), Felt::from(10), Felt::from(19), Felt::from(1), Felt::from(9)];
        let eval = horner_eval(&coefs, Felt::from(13));
        assert_eq!(eval, Felt::from(262591));
    }

    #[test]
    fn test_horner_eval_3() {
        let coefs = vec![
            Felt::from(4),
            Felt::from(10),
            Felt::from(19),
            Felt::from(1),
            Felt::from(9),
            Felt::from(99),
            Felt::from(1),
            Felt::from(7),
            Felt::from(13),
            Felt::from(2),
            Felt::from(5),
            Felt::from(7),
            Felt::from(111),
            Felt::from(1),
        ];
        let eval = horner_eval(&coefs, Felt::from(19));
        assert_eq!(eval, Felt::from_dec_str("288577899334361215").unwrap());
    }
}
