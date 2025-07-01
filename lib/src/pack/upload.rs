use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum Data {
    Bytes(Vec<Vec<u8>>),
    Structs(Vec<UploadPackData>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadPack {
    pub header: Box<[u8]>,
    pub data: Data,
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
pub fn parse_upload_pack(pack_slice: &[u8]) -> Result<UploadPack, Error> {
    let mut parsed_pack: UploadPack = match bincode::deserialize(pack_slice) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing upload pack: {e:?}");
            return Err(Error::new(ErrorKind::Other, e));
        }
    };
    let mut data_vec: Vec<UploadPackData> = Vec::new();
    match &parsed_pack.data {
        Data::Bytes(vec) => {
            for each in vec {
                match bincode::deserialize::<UploadPackData>(each) {
                    Ok(o) => data_vec.push(o),
                    Err(e) => {
                        eprintln!("Error parsing upload pack object: {e:?}");
                        return Err(Error::new(ErrorKind::Other, e));
                    }
                }
            }
        }
        Data::Structs(_upload_pack_datas) => {}
    }
    parsed_pack.data = Data::Structs(data_vec);
    Ok(parsed_pack)
}
