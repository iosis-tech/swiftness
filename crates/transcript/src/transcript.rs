use starknet_crypto::{poseidon_hash, Felt};

pub struct Transcript {
    digest: Felt,
    counter: Felt,
}

impl Transcript {
    pub fn new(digest: Felt) -> Self {
        Self { digest, counter: Felt::from(0) }
    }

    pub fn new_with_counter(digest: Felt, counter: Felt) -> Self {
        Self { digest, counter }
    }

    pub fn random_felt_to_prover(&mut self) -> Felt {
        let hash = poseidon_hash(self.digest, self.counter);
        self.counter += Felt::ONE;
        hash
    }
}
