use chrono::Local;
use sha2::{Digest, Sha256};
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Block<T: Display + Debug> {
    index: usize,
    timestamp: String,
    data: T,
    previous_hash: String,
    hash: String,
}

impl<T: Display + Debug> Block<T> {
    pub fn new(index: usize, timestamp: String, data: T) -> Self {
        let mut block = Self {
            index,
            timestamp,
            data,
            previous_hash: "".to_string(),
            hash: "".to_string(),
        };

        block.hash = block.calculate_hash();

        block
    }

    fn calculate_hash(&self) -> String {
        let mut sha256 = Sha256::new();
        sha256.update(self.index.to_string());
        sha256.update(&self.previous_hash);
        sha256.update(&self.timestamp);
        sha256.update(self.data.to_string());
        format!("{:X}", sha256.finalize())
    }
}

#[derive(Debug)]
pub struct Blockchain<T: Display + Default + Debug> {
    chain: Vec<Block<T>>,
}

impl<T: Display + Default + Debug> Blockchain<T> {
    pub fn new() -> Self {
        Self {
            chain: vec![Self::create_genesis()],
        }
    }

    fn create_genesis() -> Block<T> {
        Block::new(0, Local::today().to_string(), T::default())
    }

    fn latest_block(&self) -> &Block<T> {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, mut block: Block<T>) {
        block.previous_hash = self.latest_block().hash.clone();
        block.hash = block.calculate_hash();
        self.chain.push(block);
    }

    pub fn check_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if block.hash != block.calculate_hash() {
                return false;
            }

            if i == 0 {
                continue;
            } else if let Some(previous_block) = self.chain.get(i.saturating_sub(1)) {
                if previous_block.hash != block.previous_hash {
                    return false;
                }
            }
        }

        true
    }
}
