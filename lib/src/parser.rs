/// Parse references pack from given bytes slice
pub fn parse_refs_pack(buff: &[u8]) {
    let refs_str: &str = str::from_utf8(buff).expect("Failed to cast slice to str");
    eprint!("refs parsing {:?}", refs_str);
}
