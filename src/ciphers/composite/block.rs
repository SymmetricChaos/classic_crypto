use std::{fs::File, io::{Error, Read, Write}};

use crate::Cipher;

/// A block version of a cipher applies it to only a portion of the text each time then reads off each encrypted block.
pub struct BlockCipher<'a> {
    cipher: Box<&'a dyn Cipher>,
    block_size: usize,
}

impl BlockCipher<'_> {
    pub fn new(cipher: &dyn Cipher, block_size: usize) -> BlockCipher {
        BlockCipher{ cipher: Box::new(cipher), block_size }
    }


}

impl crate::Cipher for BlockCipher<'_> {

    fn encrypt(&self, text: &str) -> String {
        if text.chars().count() % self.block_size != 0 {
            panic!("Number of characters in the text must be a multiple of {}",self.block_size)
        }
        let symbols: Vec<char> = text.chars().collect();
        let chunks: Vec<String> = symbols.chunks(self.block_size).map(|x| x.iter().collect::<String>()).collect();
        let mut out = String::new();
        for c in chunks {
            out.push_str( &self.cipher.encrypt(&c) )
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        if text.chars().count() % self.block_size != 0 {
            panic!("Number of characters in the text must be a multiple of {}",self.block_size)
        }
        let symbols: Vec<char> = text.chars().collect();
        let chunks: Vec<String> = symbols.chunks(self.block_size).map(|x| x.iter().collect::<String>()).collect();
        let mut out = String::new();
        for c in chunks {
            out.push_str( &self.cipher.decrypt(&c) )
        }
        out
    }

    fn encrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.encrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

    fn decrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.decrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

}