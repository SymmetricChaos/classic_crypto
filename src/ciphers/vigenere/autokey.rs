use std::fmt;
use std::collections::VecDeque;





pub struct Autokey<'a> {
    /// The Autokey version of the Vigenere Cipher stretches the key by using the plaintext to encrypt itself once the key runs out. This has the advantage of never repeating.
    key_vals: Vec<usize>,
    key_name: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl Autokey<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Autokey<'a> {
        let key_name = key;
        let key_vals: Vec<usize> = key.chars().map(|x| alphabet.chars().position(|c| c == x).unwrap()).collect();

        Autokey{ key_vals, key_name, alphabet, length: alphabet.chars().count() }
    }

}

impl crate::auxiliary::Cipher for Autokey<'_> {

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

impl fmt::Display for Autokey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Autokey Cipher\nkey: {}",self.key_name)
    }
}





#[test]
fn vigenere_autokey() {
    use crate::Cipher;
    use crate::alphabets::LATIN26;
    let auto = Autokey::new("SECRET", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = auto.encrypt(plaintext);
    let decrypted = auto.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
    
}