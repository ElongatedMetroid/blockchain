use crate::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
    }
}

impl Hashable for Block {
    /// Convert Block to a set of bytes
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

impl Block {
    /// Create a new instance of a Block with the hash already calculated
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        let mut block = Self {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        };

        block.hash = block.hash();

        block
    }

    pub fn mine(&mut self) {
        // Recalculate the hash until it is less than the difficulty
        for nonce_attempt in 0..(u64::MAX) {
            // Set the nonce
            self.nonce = nonce_attempt;
            // Calculate the new hash (since the nonce is different)
            let hash = self.hash();
            // If our hash fits the difficulty
            if check_difficulty(&hash, self.difficulty) {
                // Set the hash
                self.hash = hash;
                // Break from the nonce_attempt loop
                break;
            }
        }
    }
}

/// Return true if the hash fits the difficulty (aka the difficulty is greater than the hash)
pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}
