use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadPack {
    pub data: Vec<ObjectsPackData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectsPackData {
    pub hash: [u8; 20],
    pub header: Vec<u8>,
    pub object_type: Vec<u8>,
    pub data: Vec<u8>,
}

/// Parse a slice of bytes and return an upload-pack
pub fn parse_upload_pack(pack_slice: &[u8]) -> Result<UploadPack, Error> {
    let parsed_pack: UploadPack = match bincode::deserialize(&pack_slice) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing upload pack: {e:?}");
            return Err(Error::new(ErrorKind::Other, e));
        }
    };
    Ok(parsed_pack)
}
