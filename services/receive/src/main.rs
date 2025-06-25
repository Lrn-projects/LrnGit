use std::io::{stdin, BufRead};

fn main() {
    println!("lrngit-receive-service");
    let stdin = stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
