use crate::*;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    /// Calculate the SHA256 hash of self.bytes()
    fn hash(&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
