#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod diploma_contract {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct ProofOfExistenceContract {
        proofs: Mapping<Hash, (u64, AccountId)>,
        institutions: Vec<String>,
    }

    impl ProofOfExistenceContract {
        #[ink(constructor)]
        pub fn new() -> Self {

            let mut institutions = Vec::new();
            institutions.push(String::from("UTwente"));
            institutions.push(String::from("TU Delft"));
            institutions.push(String::from("Leiden University"));
            institutions.push(String::from("Utrecht University"));
            institutions.push(String::from("VU Amsterdam"));

            Self {
                proofs: Mapping::new(),
                institutions,
            }
        }

        #[ink(message)]
        pub fn store_proof(&mut self, content_hash: Hash) {
            let timestamp = Self::env().block_timestamp();
            let sender = Self::env().caller();
            self.proofs.insert(content_hash, &(timestamp, sender));
        }

        /// Returns true if the given content hash has a proof of existence stored.
        #[ink(message)]
        pub fn check_proof(&self, content_hash: Hash) -> bool {
            self.proofs.contains(&content_hash)
        }

        #[ink(message)]
        pub fn get_institutions(&self) -> Vec<String>{
            return self.institutions.clone();
        }
    }
}
