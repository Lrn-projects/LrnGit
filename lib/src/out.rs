use std::io::Write;

/// Write framed message to anything implementing the Write trait using framed message 
///
/// Arguments:
///
/// length: length of the message in bytes slice. Use reference to_le_bytes.
/// msg: message to write in stdout as slice.
/// stdout: ptr to impl of Write.
pub fn write_framed_message_stdout(length: u32, msg: &str, stdout: &mut impl Write) {
    let length_slice = length.to_le_bytes();
    stdout.write_all(&length_slice).expect("Failed to write length to stdout");
    stdout.write_all(&msg.as_bytes()).expect("Failed to write message to stdout");
    stdout.flush().expect("Failed to flush stdout");
}
