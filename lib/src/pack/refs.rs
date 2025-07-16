/// Structure used for fast access to element in references pack
#[derive(Debug)]
pub struct ParsedRefsPack<'a> {
    pub refs: &'a str,
    pub local_commit: &'a str,
    pub origin_commit: &'a str,
}

/// Parse references pack from given bytes slice
pub fn parse_refs_pack<'a>(buff: &'a [u8]) -> ParsedRefsPack<'a> {
    let refs_str: &str=
        str::from_utf8(&buff).expect("Failed to cast buffer to owned string");
    let split: Vec<&str> = refs_str.split(" ").collect();
    ParsedRefsPack {
        refs: split[0],
        local_commit: split[1],
        origin_commit: split[2],
    }
}
