use std::fmt::{self, Debug, Formatter};
use crate::*;

pub struct Block {
    pub index: u32, 
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload,
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
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

impl Block {
    pub fn new(
        index: u32, 
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        let mut block = Self {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty,
        };

        block.hash = block.hash();

        block
    }

    pub fn mine(&mut self) {
        // Recalculate the hash until it is less than the difficulty
        for nonce_attempt in 0..(u64::MAX) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            // If our hash fits the difficulty
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                break;
            }
        }
    }
}

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}