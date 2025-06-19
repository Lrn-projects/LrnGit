use std::path::PathBuf;

use serde::Serialize;

use crate::{
    object::{
        commit,
        utils::{walk_root_tree_all_objects, walk_root_tree_content},
    },
    refs::parse_current_branch,
};

#[derive(Serialize)]
struct UploadPack {
    header: Box<[u8]>,
    data: Vec<UploadPackData>,
    footer: Box<[u8]>,
}

#[derive(Serialize)]
struct UploadPackData {
    header: Box<[u8]>,
    object_type: Box<[u8]>,
    hash: [u8; 20],
}

pub fn create_upload_pack(refs: &str, last_commit: Vec<u8>) -> Vec<u8> {
    let mut header_content: Vec<u8> = b"PUSH ".to_vec();
    header_content.extend_from_slice(refs.as_bytes());
    let header: Box<[u8]> = header_content.as_slice().to_vec().into_boxed_slice();
    // Get all objects in local repository
    let last_commit = parse_current_branch();
    let parse_commit = commit::parse_commit_by_hash(&last_commit);
    let root_tree = hex::encode(parse_commit.tree);
    let mut all_root_tree_objects: Vec<(&str, [u8; 20])> = Vec::new();
    // Contain all objects ready to be send to in upload pack
    walk_root_tree_all_objects(&root_tree, &mut PathBuf::new(), &mut all_root_tree_objects);
    all_root_tree_objects.sort();
    all_root_tree_objects.dedup();
    let mut data: Vec<UploadPackData> = Vec::new();
    for each in &all_root_tree_objects {
        let new_object: UploadPackData = UploadPackData {
            header: b"OBJECT".as_slice().to_vec().into_boxed_slice(),
            object_type: each.0.as_bytes().to_vec().into_boxed_slice(),
            hash: each.1,
        };
        data.push(new_object);
    }

    let footer: Box<[u8]> = b"END".to_vec().into_boxed_slice();
    let pack: UploadPack = UploadPack {
        header,
        data,
        footer,
    };
    bincode::serialize(&pack).expect("Failed to serialize upload pack")
}
