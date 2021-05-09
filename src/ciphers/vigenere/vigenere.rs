use std::fmt;
use std::collections::VecDeque;
use std::{ fs::File, io::{Error, Read}};


pub struct Vigenere<'a> {
    key_vals: Vec<usize>,
    key_name: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl Vigenere<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Vigenere<'a> {
        let key_name = key;
        let key_vals: Vec<usize> = key.chars().map(|x| alphabet.chars().position(|c| c == x).unwrap()).collect();
        Vigenere{ key_vals, key_name, alphabet, length: alphabet.chars().count() }
    }
}

impl crate::auxiliary::Cipher for Vigenere<'_> {

    fn encrypt(&self, text: &str) -> String {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = "".to_string();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n+k)%self.length ).unwrap() )
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() + self.length ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = "".to_string();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n-k)%self.length ).unwrap() )
        }
        out
    }

}

impl fmt::Display for Vigenere<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Cipher\nkey: {:?}",self.key_name)
    }
}





pub struct VigenereAutokey<'a> {
    key_vals: Vec<usize>,
    key_name: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl VigenereAutokey<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> VigenereAutokey<'a> {
        let key_name = key;
        let key_vals: Vec<usize> = key.chars().map(|x| alphabet.chars().position(|c| c == x).unwrap()).collect();

        VigenereAutokey{ key_vals, key_name, alphabet, length: alphabet.chars().count() }
    }

}

impl crate::auxiliary::Cipher for VigenereAutokey<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        for n in nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.alphabet.chars().nth( (n+k)%self.length ).unwrap() )
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        for n in nums {
            let k = akey.pop_front().unwrap();
            let v = (n + self.length - k ) % self.length;
            let c = self.alphabet.chars().nth( v ).unwrap();
            out.push(c);
            akey.push_back(v);
        }
        out
    }

}

impl fmt::Display for VigenereAutokey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere VigenereAutokey Cipher\nkey: {:?}",self.key_name)
    }
}





pub struct VigenereRunningKey<'a> {
    key_file: &'a str,
    alphabet: &'a str,
    length: usize,
}


impl VigenereRunningKey<'_> {
    pub fn new<'a>(key_file: &'a str, alphabet: &'a str) -> VigenereRunningKey<'a> {
        VigenereRunningKey{ key_file, alphabet, length: alphabet.chars().count() }
    }

    // This is very inefficient 
    fn encrypt(&self, text: &str) -> Result<String,Error> {
        let mut key_text = String::new();
        let mut source = File::open(self.key_file)?;
        source.read_to_string(&mut key_text)?;
        let key: Vec<usize> = key_text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();

        let mut out = String::new();

        for (n,k) in nums.iter().zip(key) {
            out.push(self.alphabet.chars().nth( (n+k)%self.length ).unwrap() )
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,Error> {
        let mut key_text = String::new();
        let mut source = File::open(self.key_file)?;
        source.read_to_string(&mut key_text)?;
        let key: Vec<usize> = key_text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();

        let mut out = String::new();

        for (n,k) in nums.iter().zip(key) {
            out.push(self.alphabet.chars().nth( (self.length+n-k)%self.length ).unwrap() )
        }
        Ok(out)
    }

}

impl fmt::Display for VigenereRunningKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Running Key Cipher\nkey file: {:?}",self.key_file)
    }
}








#[test]
fn vigenere() {
    use crate::Cipher;
    use crate::alphabets::LATIN26;
    let vig = Vigenere::new("SECRET", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = vig.encrypt(plaintext);
    let decrypted = vig.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
    
}


#[test]
fn vigenere_autokey() {
    use crate::Cipher;
    use crate::alphabets::LATIN26;
    let auto = VigenereAutokey::new("SECRET", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = auto.encrypt(plaintext);
    let decrypted = auto.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
    
}


#[test]
fn vigenere_running_key() -> Result<(),Error> {
    use crate::alphabets::LATIN26;
    let auto = VigenereRunningKey::new("dickens.txt", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = auto.encrypt(plaintext)?;
    let decrypted = auto.decrypt(&ciphertext)?;


    println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
    
    Ok(())
}