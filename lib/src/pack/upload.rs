use std::process::exit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadPack {
    pub header: Box<[u8]>,
    pub data: Vec<UploadPackData>,
    pub footer: Box<[u8]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadPackData {
    pub header: Box<[u8]>,
    pub object_type: Box<[u8]>,
    pub hash: [u8; 20],
    pub data: Box<[u8]>,
}

/// Parse a slice of bytes and return an upload-pack
pub fn parse_upload_pack(pack_slice: &[u8]) {
    let parsed_pack: UploadPack = match bincode::deserialize(pack_slice) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing tree: {e:?}");
            exit(1)
            // return Err(Box::new(e));
        }
    };
    println!("debug pack: {parsed_pack:?}");
}
