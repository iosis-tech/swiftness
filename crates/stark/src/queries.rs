use alloc::vec::Vec;
use starknet_core::types::NonZeroFelt;
use starknet_crypto::Felt;
use swiftness_air::domains::StarkDomains;
use swiftness_transcript::transcript::Transcript;

const FIELD_GENERATOR: Felt = Felt::from_hex_unchecked("0x3");
const DIVISOR: Felt = Felt::from_hex_unchecked("0x100000000000000000000000000000000");
const MAX_DOMAIN_SIZE: Felt = Felt::from_hex_unchecked("0x40");

pub fn generate_queries(
    transcript: &mut Transcript,
    n_samples: Felt,
    query_upper_bound: Felt,
) -> Vec<Felt> {
    let n: u128 = n_samples.to_biguint().try_into().unwrap();
    let mut samples: Vec<Felt> = (0..n)
        .map(|_| {
            let res = transcript.random_felt_to_prover();
            let (_, low) = res.div_rem(&NonZeroFelt::from_felt_unchecked(DIVISOR));
            let (_, sample) = low.div_rem(&NonZeroFelt::try_from(query_upper_bound).unwrap());
            sample
        })
        .collect();

    samples.sort();
    samples
}

pub fn queries_to_points(queries: &[Felt], stark_domains: &StarkDomains) -> Vec<Felt> {
    let mut points = Vec::<Felt>::new();

    // Evaluation domains of size greater than 2**64 are not supported
    assert!((stark_domains.log_eval_domain_size) <= MAX_DOMAIN_SIZE);

    // A 'log_eval_domain_size' bits index can be bit reversed using bit_reverse_u64 if it is
    // multiplied by 2**(64 - log_eval_domain_size) first.
    let shift = Felt::TWO.pow_felt(&(MAX_DOMAIN_SIZE - stark_domains.log_eval_domain_size));

    for query in queries {
        let index: u64 = (query * shift).to_bigint().try_into().unwrap();
        points.push(FIELD_GENERATOR * stark_domains.eval_generator.pow(index.reverse_bits()))
    }
    points
}
