use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
    process::{Command, exit},
};

use crate::{
    add::{
        self,
        index::{self, parse_index},
    },
    branch,
    commit::parse_commit_by_hash,
     parser,
    status::{FileStatus, FileStatusEntry},
};
use chrono::{DateTime, NaiveDateTime, Utc};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use lrncore::logs::error_log;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectHeader {
    pub types: Vec<u8>,
    pub size: usize,
}

pub fn lrngit_usage() -> &'static str {
    (r"
lrngit's cli.


Usage: lrngit command [options]


Commands:
    init            Init a local repository
    add             Add file to local repository
    commit          Commit to the local repository
    branch          Create a new branch or list all branches
    switch          Switch branch to the given one
    cat-file        Cat content of a given hash
    ls-file         Print content of the index file
    status          Show the status of the local repository
    log             Show the commit historic
    config          Manage config
    help            Show this help message
    version         Show the version

Options:

    -h, --help      Show command usage
    -v, --version   Show the current version of LrnGit
") as _
}

pub fn change_wkdir(dir: &str) {
    env::set_current_dir(dir).expect("Failed to change directory");
}

// create a new folder in objects 
pub fn add_folder(dir: &str) {
    if dir.is_empty() {
        return;
    }
    if Path::new(&format!(".lrngit/objects/{dir}")).exists() {
        return;
    }
    let new_dir_path = format!(".lrngit/objects/{dir}");
    let mut mkdir = Command::new("mkdir")
        .arg(new_dir_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to create all directories");
    let wait_mkdir = mkdir.wait().expect("Failed to wait the mkdir command");
    if !wait_mkdir.success() {
        panic!("Failed to execute the mkdir command");
    }
}

/// The function `read_blob_file` reads a compressed file, decompresses it, and prints its contents as a
/// string.
pub fn read_blob_file(hash: &str) {
    let hash_char: Vec<char> = hash.chars().collect();
    let folder: String = format!("{}{}", hash_char[0], hash_char[1]);
    let object: String = hash_char[2..].iter().collect();
    let object_path = format!(".lrngit/objects/{}/{}", &folder, &object);
    let mut read_file = fs::File::open(object_path).expect("Failed to open file");
    let mut buf = Vec::new();
    read_file
        .read_to_end(&mut buf)
        .expect("Failed to read file");
    let mut d = flate2::read::ZlibDecoder::new(buf.as_slice());
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));
    //TODO parse buffer to tree or blob struct to display
}

// Display the content of the index file
pub fn ls_file() {
    let config = add::index::parse_index();
    for each in config.entries {
        println!(
            "{:o} {} {} {}\n",
            each.mode,
            hex::encode(each.hash),
            each.flag,
            String::from_utf8_lossy(&each.path)
        );
    }
}

/**
The function `git_object_header` generates a Git object header based on the file type and content
length provided.

Arguments:

* `file_type`: The `file_type` parameter represents the type of Git object, which can be either
  "blob" or "tree".
* `content_length`: The `content_length` parameter represents the length of the content associated
  with the Git object. It is used to construct the header of the Git object based on the specified
  `file_type`.

Returns:

The function `git_object_header` returns a vector of bytes representing the header of a Git object
based on the provided `file_type` and `content_length`. If the `file_type` is "blob", it will return
a byte vector containing the header "blob {content_length}\0". If the `file_type` is "tree", it will
return a byte vector containing the header "tree
*/
pub fn git_object_header(file_type: &str, content_length: usize) -> Vec<u8> {
    match file_type {
        "blob" => format!("blob {content_length}\0").as_bytes().to_vec(),
        "tree" => format!("tree {content_length}\0").as_bytes().to_vec(),
        "commit" => format!("commit {content_length}\0").as_bytes().to_vec(),
        _ => vec![],
    }
}

/**
The `compress_file` function compresses a vector of bytes using zlib compression.

Arguments:

* `vec`: The `vec` parameter in the `compress_file` function is a vector of unsigned 8-bit integers
  (`Vec<u8>`) that represents the data of a file that you want to compress using the zlib compression
  algorithm.

Returns:

The function `compress_file` returns a `Vec<u8>` containing the compressed bytes of the input
`Vec<u8>` after compressing it using zlib compression.
*/
pub fn compress_file(vec: Vec<u8>) -> Vec<u8> {
    // compress file to zlib
    let mut compress_file = ZlibEncoder::new(Vec::new(), Compression::default());
    let compress_file_write = compress_file.write_all(&vec);
    match compress_file_write {
        Ok(_) => (),
        Err(e) => {
            lrncore::logs::error_log_with_code(
                "Failed to add file to local repository",
                &e.to_string(),
            );
            return vec![];
        }
    }
    let compressed_bytes = compress_file.finish();

    let compressed_bytes_vec: Vec<u8> = match compressed_bytes {
        Ok(v) => v,
        Err(e) => {
            lrncore::logs::error_log_with_code(
                "Failed to add file to local repository",
                &e.to_string(),
            );
            return vec![];
        }
    };
    compressed_bytes_vec
}

/**
The function `new_file_dir` creates a new file in a specified directory based on input characters.

Arguments:

* `hash_vec`: The `hash_vec` parameter is a reference to a vector of characters. The function
  new_file_dir` takes this vector as input and performs the following operations:

Returns:

The function `new_file_dir` is returning a `Result` enum with the success variant containing a
`File` if the file creation is successful, and the error variant containing a `std::io::Error` if
there is an error during the file creation process.
*/
pub fn new_file_dir(hash_vec: &[char]) -> Result<File, std::io::Error> {
    let new_folder_name = format!("{}{}", hash_vec[0], hash_vec[1]);
    add_folder(&new_folder_name);
    let new_file_name = hash_vec[2..].iter().collect::<String>().to_string();
    let new_tree_path = format!(".lrngit/objects/{new_folder_name}/{new_file_name}");
    let file: File = match File::create(&new_tree_path) {
        Ok(f) => f,
        Err(e) => {
            lrncore::logs::error_log(&format!("Failed to create new tree file: {e}"));
            return Err(e);
        }
    };
    Ok(file)
}

// The function `hash_sha1` calculates the SHA-1 hash of a given vector of bytes and returns the hash
// as an array of bytes and as a vector of characters representing the hexadecimal hash.
//
// Arguments:
//
// * `data`: The `data` parameter is a reference to a vector of unsigned 8-bit integers (`Vec<u8>`),
// which represents the data that you want to hash using the SHA-1 algorithm.
pub fn hash_sha1(data: &Vec<u8>) -> ([u8; 20], Vec<char>) {
    let mut new_hash = Sha1::new();
    new_hash.update(data);
    let hash_result = new_hash.finalize();
    let folder_hash = format!("{hash_result:#x}");
    let split_hash_result_hex = folder_hash.chars().collect::<Vec<char>>();
    (hash_result.into(), split_hash_result_hex)
}

pub fn get_file_by_hash(hash: &str) -> File {
    let split_hash: Vec<char> = hash.chars().collect();
    let folder_name: String = format!("{}{}", split_hash[0], split_hash[1]);
    let file_name: String = split_hash[2..].iter().collect::<String>().to_string();
    let path = format!(".lrngit/objects/{folder_name}/{file_name}");
    File::open(path).expect("Failed to open file")
}

/// parse git object header and return two vectors
/// first index of output vector is the header vector, second is the rest of the params buffer
pub fn split_object_header(mut buf: Vec<u8>) -> Vec<Vec<u8>> {
    // parse buffer until reach \0
    // remove header from rest of the buffer and add them in a new vec
    let mut header_bytes: Vec<u8> = Vec::new();
    for bytes in buf.clone() {
        header_bytes.push(bytes);
        if let Some(index) = buf.iter().position(|value| *value == bytes) {
            buf.remove(index);
        }
        if bytes == 0 {
            break;
        }
    }
    let mut output_vec = Vec::new();
    let new_vec = buf.clone();
    output_vec.push(header_bytes);
    output_vec.push(new_vec);
    output_vec
}

// convert a timestamp to readable datetime
pub fn timestamp_to_datetime(timestamp: i64) -> String {
    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);

    // Format the datetime how you want
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Split the given hash to return the path to the hash object
pub fn split_hash(hash: &str) -> String {
    let split_hash: Vec<char> = hash.chars().collect();
    let folder_name: String = format!("{}{}", split_hash[0], split_hash[1]);
    let file_name: String = split_hash[2..].iter().collect::<String>().to_string();
    let path = format!(".lrngit/objects/{folder_name}/{file_name}");
    path
}

/// Walkdir trough the tree object from the root tree until reach the specify path and return the
/// blob object hash. The file we want to get must be a file committed, or else the tree wont be
/// created and the function will not work.
///
/// Params:
/// root_tree: the root tree hash as &str
/// target_path: the path we want to have as tree and blob object, consider we want to get the
/// files at the end of the path.
/// current_path: mutable reference to a string to keep track of the path across recursively. It's
/// used to compare with the target_path and to get the metadata of the path.
/// hash: mutable reference to a buffer to return through a pointer the hash of the blob at the end
/// of the recursive
pub fn walk_root_tree_to_file(
    root_tree: &str,
    target_path: &str,
    current_path: &mut String,
    hash: &mut [u8; 20],
) {
    let root_tree_path = split_hash(root_tree);
    let mut root_tree_obj = File::open(root_tree_path).expect("Failed to open root tree file");
    let mut file_buff: Vec<u8> = Vec::new();
    root_tree_obj
        .read_to_end(&mut file_buff)
        .expect("Failed to read root tree content to buffer");
    let parse_root_tree =
        parser::parse_tree_entries_obj(file_buff).expect("Failed to parse tree entries");
    for each in parse_root_tree {
        match current_path.is_empty() {
            true => *current_path = str::from_utf8(&each.name).unwrap().to_string(),
            false => {
                current_path.push('/');
                current_path.push_str(str::from_utf8(&each.name).unwrap());
                // current_path.push_str(str::from_utf8(&each.name).unwrap());
            }
        }
        let metadata = std::fs::metadata(&current_path).unwrap();
        if metadata.is_dir() {
            walk_root_tree_to_file(&hex::encode(each.hash), target_path, current_path, hash);
        };
        if metadata.is_file() {
            // Deferencing ptr to assign mut value
            *hash = each.hash;
        }
    }
}

// Check in the index file if a file has been modified since it has been added to the index
//
// Params:
// files_path: path to the file we want to check
//
// Return a FileStatusEntry structure containing the file path and the status.
pub fn check_modified_file(files_path: &str) -> FileStatusEntry {
    let index = index::parse_index();
    let mut index_entries = index.entries;
    let mut file_status: FileStatusEntry = FileStatusEntry {
        file: "".to_owned(),
        status: FileStatus::Untracked,
    };

    let file_metadata = fs::metadata(files_path).expect("Failed to get file metadata");

    if let Some(pos) = index_entries
        .iter()
        .position(|x| str::from_utf8(&x.path).unwrap() == files_path)
    {
        let entry = index_entries.remove(pos);
        if file_metadata.mtime() as u32 != entry.mtime
            || file_metadata.len() as u32 != entry.file_size
        {
            file_status = FileStatusEntry {
                file: files_path.to_owned(),
                status: FileStatus::Modify,
            }
        } else {
            file_status = check_file_staged(files_path);
        }
    }
    file_status
}

// Check if a file is staged or just modified by comparing hash from last commit with the one from
// index
fn check_file_staged(file_path: &str) -> FileStatusEntry {
    let last_commit = branch::parse_current_branch();
    let parse_commit = parse_commit_by_hash(&last_commit);
    let mut file_hash: [u8; 20] = [0u8; 20];
    // Get the hash of the file from last commit to check if there's change on disk
    walk_root_tree_to_file(
        &hex::encode(parse_commit.tree),
        file_path,
        &mut String::new(),
        &mut file_hash,
    );
    let mut index = parse_index();
    if let Some(pos) = index
        .entries
        .iter()
        .position(|x| String::from_utf8_lossy(&x.path) == file_path)
    {
        let entry = index.entries.remove(pos);
        let disk_hash = add::helpers::calculate_file_hash_and_blob(file_path)
            .expect("Failed to get hash from file path");
        // if entry.hash != file_hash && entry.hash == disk_hash
        if entry.hash != file_hash && entry.hash == disk_hash.hash {
            FileStatusEntry {
                file: file_path.to_owned(),
                status: FileStatus::Staged,
            }
        } else {
            FileStatusEntry {
                file: file_path.to_owned(),
                status: FileStatus::Tracked,
            }
        }
    } else {
        error_log("Error checking file status");
        exit(1)
    }
}
