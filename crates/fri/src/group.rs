use alloc::vec;
use alloc::vec::Vec;
use starknet_crypto::Felt;

// Returns the elements of the multiplicative subgroup of order 16, in bit-reversed order for the
// cairo prime field. Note that the first 2^k elements correspond to the group of size 2^k.
pub fn get_fri_group() -> [Felt; 16] {
    [
        Felt::from_hex_unchecked("0x1"),
        Felt::from_hex_unchecked(
            "0x800000000000011000000000000000000000000000000000000000000000000",
        ),
        Felt::from_hex_unchecked(
            "0x625023929a2995b533120664329f8c7c5268e56ac8320da2a616626f41337e3",
        ),
        Felt::from_hex_unchecked(
            "0x1dafdc6d65d66b5accedf99bcd607383ad971a9537cdf25d59e99d90becc81e",
        ),
        Felt::from_hex_unchecked(
            "0x63365fe0de874d9c90adb1e2f9c676e98c62155e4412e873ada5e1dee6feebb",
        ),
        Felt::from_hex_unchecked(
            "0x1cc9a01f2178b3736f524e1d06398916739deaa1bbed178c525a1e211901146",
        ),
        Felt::from_hex_unchecked(
            "0x3b912c31d6a226e4a15988c6b7ec1915474043aac68553537192090b43635cd",
        ),
        Felt::from_hex_unchecked(
            "0x446ed3ce295dda2b5ea677394813e6eab8bfbc55397aacac8e6df6f4bc9ca34",
        ),
        Felt::from_hex_unchecked(
            "0x5ec467b88826aba4537602d514425f3b0bdf467bbf302458337c45f6021e539",
        ),
        Felt::from_hex_unchecked(
            "0x213b984777d9556bac89fd2aebbda0c4f420b98440cfdba7cc83ba09fde1ac8",
        ),
        Felt::from_hex_unchecked(
            "0x5ce3fa16c35cb4da537753675ca3276ead24059dddea2ca47c36587e5a538d1",
        ),
        Felt::from_hex_unchecked(
            "0x231c05e93ca34c35ac88ac98a35cd89152dbfa622215d35b83c9a781a5ac730",
        ),
        Felt::from_hex_unchecked(
            "0x00b54759e8c46e1258dc80f091e6f3be387888015452ce5f0ca09ce9e571f52",
        ),
        Felt::from_hex_unchecked(
            "0x7f4ab8a6173b92fda7237f0f6e190c41c78777feabad31a0f35f63161a8e0af",
        ),
        Felt::from_hex_unchecked(
            "0x23c12f3909539339b83645c1b8de3e14ebfee15c2e8b3ad2867e3a47eba558c",
        ),
        Felt::from_hex_unchecked(
            "0x5c3ed0c6f6ac6dd647c9ba3e4721c1eb14011ea3d174c52d7981c5b8145aa75",
        ),
    ]
}
