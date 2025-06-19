 pub struct FileHashBlob {
     pub blob: Vec<u8>,
     pub hash: [u8; 20],
     pub hash_split: Vec<char>,
 }

#[allow(dead_code)]
 struct BlobObject {
     // "blob <size>\0" in binary
     header: Vec<u8>,
     content: Vec<u8>,
 }
