/*
Module handling all the add command related functions
*/

use crate::object::blob::add_blob;

pub fn add_to_local_repo(arg: String) {
    let _folder_vec: Vec<&str> = if arg.contains("/") {
        let folder_split: Vec<&str> = arg.split("/").collect();
        folder_split
    } else {
        vec![&arg]
    };
    add_blob(&arg);
}

