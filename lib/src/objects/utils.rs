use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
 pub struct ObjectHeader {
     pub types: Vec<u8>,
     pub size: usize,
 }
