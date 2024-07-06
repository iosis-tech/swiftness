use starknet_crypto::{poseidon_hash, poseidon_hash_many, Felt};

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

    pub fn digest(&self) -> &Felt {
        &self.digest
    }

    pub fn counter(&self) -> &Felt {
        &self.counter
    }

    pub fn random_felt_to_prover(&mut self) -> Felt {
        let hash = poseidon_hash(self.digest, self.counter);
        self.counter += Felt::ONE;
        hash
    }

    pub fn random_felts_to_prover(&mut self, mut len: Felt) -> Vec<Felt> {
        let mut res = Vec::new();
        while len > Felt::ZERO {
            res.push(self.random_felt_to_prover());
            len -= Felt::ONE
        }
        res
    }

    pub fn read_felt_from_prover(&mut self, val: &Felt) {
        let hash = poseidon_hash_many([&(self.digest + Felt::ONE), val]);
        self.digest = hash;
        self.counter = Felt::ZERO;
    }

    pub fn read_felt_vector_from_prover(&mut self, val: &[Felt]) {
        let hash = poseidon_hash_many(vec![&(self.digest + Felt::ONE)].into_iter().chain(val));
        self.digest = hash;
        self.counter = Felt::ZERO;
    }

    pub fn read_uint64_from_prover(&mut self, val: u64) {
        self.read_felt_from_prover(&Felt::from(val))
    }
}
