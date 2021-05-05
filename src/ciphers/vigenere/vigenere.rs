use std::fmt;



pub struct Vigenere {
    key: Vec<usize>,
    key_name: String,
    alphabet: String,
    length: usize,
}

impl Vigenere {
    pub fn new(key: &str, alpha: &str) -> Vigenere {
        let key_name = key.to_string();
        let key: Vec<usize> = key.chars().map(|x| alpha.chars().position(|c| c == x).unwrap()).collect();
        Vigenere{ key, key_name, alphabet: alpha.to_string(), length: alpha.chars().count() }
    }
}

impl crate::auxiliary::Cipher for Vigenere {

    fn encrypt(&self, text: &str) -> String {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key.iter().cycle();
        let mut out = "".to_string();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n+k)%self.length ).unwrap() )
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() + self.length ).collect();
        let ckey = self.key.iter().cycle();
        let mut out = "".to_string();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n-k)%self.length ).unwrap() )
        }
        out
    }

}

impl fmt::Display for Vigenere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Cipher\nkey: {:?}",self.key_name)
    }
}

#[test]
fn vigenere() {
    use crate::Cipher;
    use crate::alphabets::LATIN26;
    let vig = Vigenere::new("SECRET", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = vig.encrypt(plaintext);
    let cleartext = vig.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
    
}