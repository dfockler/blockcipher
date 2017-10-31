
pub struct Cipher;

impl Cipher {
    fn decode(ciphertext: &[u8]) -> Vec<u8> {
        unimplemented!();
    }

    pub fn encode(plaintext: &[u8]) {
        // split_into_blocks()
        // or iterate over blocks of correct size
        for block in plaintext.chunks(16) {
            println!("{:?}", block);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        let text = "hello, world! What are you up to today"; 
        Cipher::encode(text.as_bytes());
    }
}
