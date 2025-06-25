use std::io::{stdin, BufRead};

fn main() {
    println!("[SERVICE] lrngit-receive");
    let stdin = stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
