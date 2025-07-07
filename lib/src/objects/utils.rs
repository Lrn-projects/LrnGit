use std::{
    fs::{self, read_dir},
    io::Read,
    path::PathBuf,
    str::FromStr,
};

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

pub fn get_all_object(path: &str, object_vec: &mut Vec<PathBuf>) {
    let object_path =
        PathBuf::from_str(path).expect("Failed to create new pathbuf from object path");
    for object_read_dir in read_dir(object_path).unwrap() {
        let object = object_read_dir.expect("Failed to get object in objects");
        let object_metadata = object.metadata().expect("Failed to get object metadata");
        if object_metadata.is_file() {
            object_vec.push(object.path());
        } else if object_metadata.is_dir() {
            get_all_object(
                object
                    .path()
                    .as_os_str()
                    .to_str()
                    .expect("Failed to cast os_str to str"),
                object_vec,
            );
        }
    }
}

/// Parse git object header and return two vectors
/// first index of output vector is the header vector, second is the rest of the params buffer
pub fn split_object_header(mut buf: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    // Parse buffer until reach \0
    // remove header from rest of the buffer and add them in a new vec
    let mut header_bytes: Vec<u8> = Vec::new();
    for bytes in buf.clone() {
        header_bytes.push(bytes);
        if let Some(index) = buf.iter().position(|value| *value == bytes) {
            buf.remove(index);
        }
        if bytes == 0 {
            break;
        }
    }
    let new_vec = buf.clone();
    (header_bytes, new_vec)
}

pub fn parse_hash_objects(objects_path: Vec<PathBuf>) -> Vec<String> {
    let mut hash_vec: Vec<String> = Vec::new();
    for each_path in objects_path {
        let split: Vec<&str> = each_path
            .to_str()
            .expect("Failed to cast pathbuf to str")
            .split("/")
            .collect();
        let hash: String = format!("{}{}", split[split.len() - 2], split[split.len() - 1]);
        hash_vec.push(hash);
    }
    hash_vec
}

/// Parse given object header and return the object type and the size of the object content
pub fn parse_object_header(hash: &str) -> (String, usize) {
    let object_path: String = split_hash(hash);
    let mut read_file = fs::File::open(object_path).expect("Failed to open file");
    let mut buf = Vec::new();
    read_file
        .read_to_end(&mut buf)
        .expect("Failed to read file");
    let mut d = flate2::read::ZlibDecoder::new(buf.as_slice());
    let mut buffer: Vec<u8> = Vec::new();
    d.read_to_end(&mut buffer).unwrap();
    let (mut object_header_buff, _) = split_object_header(buffer);
    object_header_buff.pop();
    let header_str = str::from_utf8(&object_header_buff).expect("Failed to cast buffer to str");
    let split: Vec<&str> = header_str.split(" ").collect();
    println!("debug: {:?}", split);
    (
        split[0].to_owned(),
        split[1]
            .parse::<usize>()
            .expect("Failed to cast str to usize"),
    )
}
