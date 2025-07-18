use std::{fs::OpenOptions, io::Write};

use lrngitcore::pack::{refs::ParsedRefsPack, upload::UploadPack};

/// Update HEAD file and given refs from 'refs' parameter. Check if there's no conflict and if
/// there is all mandatory objects to rebuild historic
pub fn update_refs(refs: ParsedRefsPack, pack: UploadPack) {
    // Update HEAD
    let mut head: File = OpenOptions::new()
        .read(false)
        .write(true)
        .append(false)
        .open("HEAD")
        .expect("Failed to open HEAD file");
    let updated_head: &str = &String::from("ref: ") + refs.refs;
    head.write_all(updated_head.as_bytes())
        .expect("Failed to write updated HEAD content");
    // Update refs/heads
    let mut ref_head: File = OpenOptions::new()
        .read(false)
        .write(true)
        .append(false)
        .open(refs.refs)
        .expect("Failed to open given refs/heads file");
    ref_head
        .write_all(refs.local_commit.as_bytes())
        .expect("Failed to update refs/heads with last commit");
    // 
}
