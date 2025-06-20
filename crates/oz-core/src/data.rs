use borsh::{BorshDeserialize, BorshSerialize};
use thiserror::Error;

pub fn hash(fact: &impl BorshSerialize) -> Result<[u8; 32], DataError> {
    let serialized = borsh::to_vec(fact).map_err(DataError::SerializationError)?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(&serialized);
    let hash = hasher.finalize();
    Ok(*hash.as_bytes())
}

#[derive(Error, Debug)]
#[error("Data layer error")]
pub enum DataError {
    #[error("Serialization error: {0}")]
    SerializationError(borsh::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_function() {
        #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
        struct MyFact {
            id: u32,
        }
        let fact1 = MyFact { id: 42 };
        let fact2 = MyFact { id: 43 };
        let hash1 = hash(&fact1).expect("Hashing failed");
        let hash2 = hash(&fact1).expect("Hashing failed");
        let hash3 = hash(&fact2).expect("Hashing failed");
        assert_eq!(hash1.len(), 32, "Hash should be 32 bytes");
        assert_eq!(hash1, hash2, "Hashing the same value should be deterministic");
        assert_ne!(hash1, hash3, "Different values should produce different hashes");
    }
}