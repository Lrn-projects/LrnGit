use serde::{Deserialize, Serialize};
/// The `TreeEntry` struct in Rust represents an entry in a tree object with mode, name, and SHA-1 hash.
 ///
 /// Properties:
 ///
 /// * `mode`: The `mode` property in the `TreeEntry` struct represents the file mode or permissions of
 ///   the entry. It is typically a 32-bit unsigned integer that specifies the filetype and permissions,
 ///   such as whether the entry is a file, directory, or symbolic link, and the read, write,
 ///   example: if the mode is `40000` it's a folder, else if it's `100644` it's a blob,
 ///   160000 would be a commit
 /// * `name`: The `name` property in the `TreeEntry` struct represents the name of the entry in the
 ///   tree. It is of type `String` and stores the name of the entry.
 /// * `hash`: The `hash` property in the `TreeEntry` struct is an array of 20 unsigned 8-bit integers
 ///   (bytes). This array is used to store the SHA-1 hash value of the file or directory represented by
 ///   the `TreeEntry`. The SHA-1 hash is typically used to
 #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Ord, Eq)]
 #[allow(dead_code)]
 pub struct TreeEntry {
     pub mode: u32,
     pub name: Vec<u8>,
     pub hash: [u8; 20],
 }

 #[derive(Debug, Deserialize, Serialize, Clone)]
 #[allow(dead_code)]
 pub struct Tree {
     pub header: Vec<u8>,
     pub entries: Vec<TreeEntry>,
 }

 pub const SYM: u32 = 0o120000;
 pub const DIR: u32 = 0o040000;
 pub const EXE: u32 = 0o100755;
 pub const RWO: u32 = 0o100644;
