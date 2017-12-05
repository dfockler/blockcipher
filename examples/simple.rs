extern crate blockcipher;

use blockcipher::*;

// Basic example
fn main() {
    let x = "hello, world! What are you up to today";
    Cipher::encode(x.as_bytes());
}
