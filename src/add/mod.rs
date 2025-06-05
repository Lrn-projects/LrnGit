/*
Module handling all the add command related functions
*/
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::MetadataExt;

use crate::utils;
use crate::object::blob::add_blob;

pub mod helpers;
use crate::object::index;

pub fn add_to_local_repo(arg: String) {
    let _folder_vec: Vec<&str> = if arg.contains("/") {
        let folder_split: Vec<&str> = arg.split("/").collect();
        folder_split
    } else {
        vec![&arg]
    };
    add_blob(&arg);
}

