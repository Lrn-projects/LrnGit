use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
 pub struct ObjectHeader {
     pub types: Vec<u8>,
     pub size: usize,
 }

/// Split the given hash to return the path to the hash object
pub fn split_hash(hash: &str) -> String {
    let split_hash: Vec<char> = hash.chars().collect();
    let folder_name: String = format!("{}{}", split_hash[0], split_hash[1]);
    let file_name: String = split_hash[2..].iter().collect::<String>().to_string();
    let path = format!(".lrngit/objects/{folder_name}/{file_name}");
    path
}


