use std::{
    fs::File,
    io::Read,
    path::PathBuf,
};

use lrngitcore::{
    objects::utils::{get_all_object, parse_hash_objects, parse_object_header},
    pack::upload::{ObjectsPackData, UploadPack},
};

use crate::{
    object::{
        commit,
        utils::{get_file_by_hash, walk_root_tree_all_objects},
    },
    refs::parse_current_branch,
};

pub fn create_upload_pack(refs: &str, _last_commit: Vec<u8>) -> Vec<u8> {
    let mut header_content: Vec<u8> = b"PUSH ".to_vec();
    // Add refs at end of the header
    header_content.extend_from_slice(refs.as_bytes());
    // Get all objects in local repository
    let last_commit = parse_current_branch();
    let parse_commit = commit::parse_commit_by_hash(&last_commit);
    let root_tree = hex::encode(parse_commit.tree);
    let mut all_root_tree_objects: Vec<(&str, [u8; 20])> = Vec::new();
    // Contain all objects ready to be send to in upload pack
    walk_root_tree_all_objects(&root_tree, &mut PathBuf::new(), &mut all_root_tree_objects);
    let mut object_vec: Vec<PathBuf> = Vec::new();
    get_all_object(".lrngit/objects/", &mut object_vec);
    let hash = parse_hash_objects(object_vec);
    // for each in hash {
    //     let (object_type,_) = parse_object_header(&each);
    //     println!("header type: {:?}", object_type);
    // }
    all_root_tree_objects.sort();
    all_root_tree_objects.dedup();
    let mut object_vec: Vec<ObjectsPackData> = Vec::new();
    for each in hash {
        let mut file: File = get_file_by_hash(&each);
        let (object_type, _) = parse_object_header(&each);
        let mut file_buff: Vec<u8> = Vec::new();
        file.read_to_end(&mut file_buff)
            .expect("Failed to read file content");
        let new_object: ObjectsPackData = ObjectsPackData {
            header: b"OBJECT".as_slice().to_vec(),
            object_type: object_type.as_bytes().to_vec(),
            hash: {
                let mut buffer = [0u8; 20];
                let bytes = each.as_bytes();
                // Copy max 20 bytes from string buffer
                let len = bytes.len().min(20);
                buffer[..len].copy_from_slice(&bytes[..len]);
                buffer
            },
            data: file_buff,
        };
        object_vec.push(new_object);
    }
    let footer: Vec<u8> = b"END".to_vec();
    let pack: UploadPack = UploadPack {
        header: header_content,
        data: object_vec,
        footer,
    };
    let upload_pack: Vec<u8> = bincode::serialize(&pack).expect("Failed to serialize upload pack");
    upload_pack
}
