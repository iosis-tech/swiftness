use crate::consts::{FELT_0, FELT_1, FELT_2};
use starknet_crypto::Felt;

// The cumulative value is defined using the next recursive formula:
//   r_1 = 1, r_{j+1} = r_j * (1 + z * u_j) + alpha * u_j^2
// where u_j = Dilute(j, spacing, n_bits) - Dilute(j-1, spacing, n_bits)
// and we want to compute the final value r_{2^n_bits}.
// Note that u_j depends only on the number of trailing zeros in the binary representation of j.
// Specifically, u_{(1+2k)*2^i} = u_{2^i} = u_{2^{i-1}} + 2^{i*spacing} - 2^{(i-1)*spacing + 1}.
//
// The recursive formula can be reduced to a nonrecursive form:
//   r_j = prod_{n=1..j-1}(1+z*u_n) + alpha*sum_{n=1..j-1}(u_n^2 * prod_{m=n+1..j-1}(1+z*u_m))
//
// We rewrite this equation to generate a recursive formula that converges in log(j) steps:
// Denote:
//   p_i = prod_{n=1..2^i-1}(1+z*u_n)
//   q_i = sum_{n=1..2^i-1}(u_n^2 * prod_{m=n+1..2^i-1}(1+z*u_m))
//   x_i = u_{2^i}.
//
// Clearly
//   r_{2^i} = p_i + alpha * q_i.
// Moreover,
//   p_i = p_{i-1} * (1 + z * x_{i-1}) * p_{i-1}
//   q_i = q_{i-1} * (1 + z * x_{i-1}) * p_{i-1} + x_{i-1}^2 * p_{i-1} + q_{i-1}
//
// Now we can compute p_{n_bits} and q_{n_bits} in just n_bits recursive steps and we are done.
pub fn get_diluted_product(n_bits: Felt, spacing: Felt, z: Felt, alpha: Felt) -> Felt {
    let diff_multiplier = FELT_2.pow_felt(&spacing);
    let mut diff_x: Felt = diff_multiplier - FELT_2;
    let mut x: Felt = FELT_1;
    let mut p: Felt = z + FELT_1;
    let mut q: Felt = FELT_1;

    let mut i = FELT_0;
    loop {
        if i == n_bits - FELT_1 {
            break p + q * alpha;
        }

        x += diff_x;
        diff_x *= diff_multiplier;
        let x_p = x * p;
        let y = p + z * x_p;
        q = q * y + x * x_p + q;
        p *= y;

        i += FELT_1;
    }
}
