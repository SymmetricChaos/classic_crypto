use std::fmt;

use crate::auxiliary::mul_inv;

/// The Affine Cipher augments the Caesar Cipher by including a multiplicative aspect to the key.
pub struct Affine {
    key1: usize,
    key2: usize,
    key2_inv: usize,
    alphabet: String,
    length: usize
}

impl Affine {
    pub fn new(key: (usize,usize), alphabet: &str) -> Affine {
        let key1 = key.0;
        let key2 = key.1;
        let length = alphabet.chars().count();
        let key2_inv = match mul_inv(key2, length) {
            Some(m) => m,
            None => panic!("{} has no multiplicative inverse modulo {}",key2,length),
        };

        Affine{ key1, key2, key2_inv, alphabet: alphabet.to_string(), length }
    }

    fn char_to_val(&self, c: char) -> usize {
        self.alphabet.chars().position(|x| x == c).unwrap()
    }

    fn val_to_char(&self, v: usize) -> char {
        self.alphabet.chars().nth(v).unwrap()
    }



}

impl crate::auxiliary::Cipher for Affine {

    fn encrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = (self.char_to_val(s) * self.key2 + self.key1) % self.length;
            out.push(self.val_to_char(n))
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = ((self.char_to_val(s) + self.length - self.key1) * self.key2_inv) % self.length;
            out.push(self.val_to_char(n))
        }
        out
    }

}

impl fmt::Display for Affine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Affine Cipher\nkey: ({},{})",self.key1,self.key2)
    }
}

#[test]
fn affine() {
    use crate::alphabets::LATIN26;
    use crate::Cipher;
    let aff = Affine::new((1,3), LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = aff.encrypt(plaintext);
    let decrypted = aff.decrypt(&ciphertext);

    assert_eq!(&ciphertext,"GWNXJZHFEARPOQRSCJLUDRMNAGWNIBYVKRT");
    assert_eq!(&decrypted,"THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG");
    
}