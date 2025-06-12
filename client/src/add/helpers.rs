/*
Helper module for the add module, contain useful pub function
*/
#![allow(dead_code)]
use std::fs::{self};

use crate::utils::{self};

pub fn check_objects_exist(path: &str) -> bool {
    fs::exists(path).unwrap()
}
