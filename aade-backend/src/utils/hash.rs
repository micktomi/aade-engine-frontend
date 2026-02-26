use sha2::{Sha256, Digest};

pub fn calculate_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}
