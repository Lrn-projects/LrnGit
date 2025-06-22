use serde::Serialize;

#[derive(Serialize)]
pub struct UploadPack {
    pub header: Box<[u8]>,
    pub data: Vec<UploadPackData>,
    pub footer: Box<[u8]>,
}

#[derive(Serialize)]
pub struct UploadPackData {
    pub header: Box<[u8]>,
    pub object_type: Box<[u8]>,
    pub hash: [u8; 20],
    pub data: Box<[u8]>
}


