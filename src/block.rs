use crate::util;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub hash: Vec<u8>,
    pub difficulty: u64,
    pub nonce: u64,
    pub payload: String,
}

impl Block {
    pub fn to_string(&self) -> String {
        return format!(
            "Block:\n -Index: {}\n -Timestamp: {}\n -Hash: {}\n -Nonce: {}\n -Difficulty: {}\n -Payload: {}",
            &self.index,
            &self.timestamp,
            hex::encode(&self.hash),
            &self.nonce,
            &self.difficulty,
            &self.payload
        );
    }

    pub fn hash(&self) -> Vec<u8> {
        return crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.get_bytes());
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&util::u64_bytes(&self.index));
        bytes.extend(&util::u128_bytes(&self.timestamp));
        bytes.extend(&util::u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        return bytes;
    }

    pub fn find_hash(&mut self) {
        let mut count = 0;
        self.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        for new_nonce in 0..u64::max_value() {
            self.nonce = new_nonce;
            self.hash = self.hash();
            if self.difficulty > util::hash_to_difficulty_bytes(&self.hash) {
                return;
            }

            count = count + 1;
            if count % 1000 == 0 {
                println!("Finding hash {}", count);
            }
        }
        self.hash = vec![];
    }
}
