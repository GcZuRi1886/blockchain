use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String
}

impl Block {
    fn new(index: u32, timestamp: u64, data: String, previous_hash: String) -> Block {
        return Block{
            index,
            timestamp,
            data: data.clone(),
            previous_hash: previous_hash.clone(),
            hash: generate_hash(index, timestamp, data, previous_hash)
        };
    }
    
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let mut owned_string: String = self.index.to_string().to_owned();
        owned_string.push_str(&self.timestamp.to_string());
        owned_string.push_str(&self.data);
        owned_string.push_str(&self.previous_hash);
        let concated_string = owned_string.as_bytes();
    
        hasher.update(concated_string);
    
        return format!("{:X}", hasher.finalize());
    }
}

struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::new()
        };
        blockchain.chain.push(blockchain.create_genesis_block());
        return blockchain;
    }

    fn create_genesis_block(&self) -> Block {
        return Block::new(0, get_time(), "Genesis Block".to_string(), "0".to_string())
    }

    fn add_block(&mut self, timestamp: u64, data: String) {
        let index = self.chain.len();
        let previous_hash = self.chain[index-1].hash.clone();
        let new_block = Block::new(index.try_into().expect("Weird number"), timestamp, data, previous_hash);
        self.chain.push(new_block)
    }

    fn is_valid(&self) -> bool {
        for (i, el) in self.chain.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let prev_el = &self.chain[i-1];

            if el.hash != el.calculate_hash() {
                return false;
            }

            if el.previous_hash != prev_el.hash {
                return false;
            }
        }
        return true;
    }

    fn print(&self) {
        for el in self.chain.iter() {
            println!("Index: {}", el.index);
            println!("Timestamp: {}", el.timestamp);
            println!("Data: {}", el.data);
            println!("Previous Hash: {}", el.previous_hash);
            println!("Hash: {}", el.hash);
            println!("----------------------------------")
        }
    }
}


fn get_time() -> u64 {
    let now = SystemTime::now();
    return now.duration_since(UNIX_EPOCH).expect("Magic").as_secs();
}

fn generate_hash(index: u32, timestamp: u64, data: String, previous_hash: String) -> String {
    let mut hasher = Sha256::new();
    let mut owned_string: String = index.to_string().to_owned();
    owned_string.push_str(&timestamp.to_string());
    owned_string.push_str(&data);
    owned_string.push_str(&previous_hash);
    let concated_string = owned_string.as_bytes();

    hasher.update(concated_string);

    return format!("{:X}", hasher.finalize());
}

fn main() {
    let mut blockchain: Blockchain = Blockchain::new();
    blockchain.add_block(get_time(), "I'm cool".to_string());
    blockchain.add_block(get_time(), "Next element".to_string());
    blockchain.add_block(get_time(), "Another block".to_string());
    blockchain.add_block(get_time(), "Something else".to_string());
    if blockchain.is_valid() {
        println!("Blockchain is valid");
        println!("----------------------------------");
        blockchain.print();
    }
}