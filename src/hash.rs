use serde::Serialize;
use sha3::{Digest,Keccak256};


pub fn keccak_hash<T>(data: &T) -> String
where
    T: ?Sized + Serialize + AsRef<[u8]>,
{
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hex_r = hex::encode(result);
    hex_r
}

