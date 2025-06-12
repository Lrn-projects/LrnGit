use std::collections::HashMap;

pub type BatchIndexEntriesMap = HashMap<(String, usize), Vec<(String, u32, [u8; 20])>>;
pub type BatchIndexEntriesVec = Vec<((String, usize), Vec<(String, u32, [u8; 20])>)>;
pub type BatchIndexEntriesTuple = ((String, usize), Vec<(String, u32, [u8; 20])>);
