use std::{cell::Cell, fmt, fs::File, io::{Error, Read, Write}};

/// The Alberti Cipher Disk was an early polyalphabetic substitution cipher that recognized the need for irregular alphabet changes. It does this by including instructions to change the alphabet within the message itself
pub struct Alberti {
    fixed: String,
    moving: String,
    index: Cell<usize>,
    length: usize,
}


impl Alberti {
    pub fn new(fixed: &str, moving: &str) -> Alberti {
        for s in fixed.chars() {
            if moving.contains(s) {
                panic!("the fixed and moving alphabets must be entirely distinct")
            }
        }

        if fixed.chars().count() != moving.chars().count() {
            panic!("the fixed and moving alphabets must be the same length")
        }

        Alberti{ fixed: fixed.to_string(), moving: moving.to_string(), index: Cell::new(0), length: fixed.chars().count() }
    }

    pub fn set_position(&self, symbol: char) {
        self.index.set(self.fixed.chars().position(|x| x == symbol).unwrap())
    }

    fn fixed_symbol_position(&self, symbol: char) -> usize {
        self.fixed.chars().position(|x| x == symbol).unwrap()
    }

    fn moving_symbol_position(&self, symbol: char) -> usize {
        self.moving.chars().position(|x| x == symbol).unwrap()
    }
}

impl crate::Cipher for Alberti {
    
    // The starting index must be set with the .set_position() method or this is effectively just a really complex code
    // The starting index changes during computation but is then reset to whatever it started at
    fn encrypt(&self, text: &str) -> String {
        let start_index = self.index.get();
        let symbols = text.chars();
        let mut out = String::new();
        for s in symbols {
            let p = (self.fixed_symbol_position(s) + self.index.get()) % self.length;
            let f = self.moving.chars().nth(p).unwrap();
            out.push(f);
            if s.is_ascii_digit() {
                let pos = (self.index.get() + (s.to_digit(10).unwrap() as usize)) % self.length;
                self.index.set(pos);
            }
        }
        self.index.set(start_index);
        out
    }

    // The starting index changes during computation but is then reset to whatever it started at
    fn decrypt(&self, text: &str) -> String {
        let start_index = self.index.get();
        let symbols = text.chars();
        let mut out = String::new();
        for s in symbols {
            let p = (self.length + self.moving_symbol_position(s) - self.index.get()) % self.length;
            let f = self.fixed.chars().nth(p).unwrap();
            if f.is_ascii_digit() {
                let pos = (self.length + self.index.get() + (f.to_digit(10).unwrap() as usize)) % self.length;
                self.index.set(pos);
            }
            out.push(f)
        }
        self.index.set(start_index);
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

impl fmt::Display for Alberti {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alberti Cipher Disk\n{}\n{}",self.fixed,self.moving)
    }
}

#[test]
fn alberti() {
    use crate::Cipher;
    use crate::alphabets::LATIN36;

    let alberti = Alberti::new(LATIN36, "knoxw)!gf&hu^dqjys%@ltvrcbm#zp($ie*a");


    let plaintext = "THE1QUICK3BROWN7FOX1JUMPS2OVER5THE5LAZY1DOG";
    alberti.set_position('M');
    let ciphertext = alberti.encrypt(plaintext);
    alberti.set_position('M');
    let decrypted = alberti.decrypt(&ciphertext);

    assert_eq!(ciphertext,"$@yxpetjr!se(opdznhqefkx!ywu(gtqoamukbc#wjg");
    assert_eq!(decrypted, "THE1QUICK3BROWN7FOX1JUMPS2OVER5THE5LAZY1DOG");
}