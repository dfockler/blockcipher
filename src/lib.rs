extern crate block_cipher_trait;
extern crate generic_array;

use block_cipher_trait::BlockCipher;
use generic_array::{GenericArray};
use generic_array::typenum::{U1, U16, U256};

pub struct FeistelCipher;

impl BlockCipher for FeistelCipher {
    type KeySize = U256;
    type BlockSize = U16;
    type ParBlocks = U1;

    fn new(key: &GenericArray<u8, Self::KeySize>) -> Self {
        FeistelCipher {}
    }

    fn encrypt_block(&self, block: &mut GenericArray<u8, Self::BlockSize>) {

    }

   /// Decrypt block in-place
   fn decrypt_block(&self, block: &mut GenericArray<u8, Self::BlockSize>) {

   }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        let text = "hello, world! What are you up to today";
        Feistel::encrypt_block(text.as_bytes());
    }
}
