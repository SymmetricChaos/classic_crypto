use crate::Cipher;

/// A composite cipher applies several ciphers in succession.
pub struct CompositeCipher<'a> {
    ciphers: Vec<Box<&'a dyn Cipher>>,
}


impl CompositeCipher<'_> {
    pub fn new(ciphers: Vec<&dyn Cipher>) -> CompositeCipher {
        let mut cipher_vec = Vec::new();
        for c in ciphers {
            cipher_vec.push(Box::new(c))
        }
        CompositeCipher{ ciphers: cipher_vec }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let mut out = text.to_string();
        for cipher in self.ciphers.iter() {
            out = cipher.encrypt(&out)
        }
        out
    }

    pub fn decrypt(&self, text: &str) -> String {
        let mut out = text.to_string();
        for cipher in self.ciphers.iter().rev() {
            out = cipher.decrypt(&out)
        }
        out
    }

    pub fn encrypt_steps(&self, text: &str) -> Vec<String> {
        let mut out = vec![text.to_string()];
        let mut partial = text.to_string();
        for cipher in self.ciphers.iter() {
            partial = cipher.encrypt(&partial);
            out.push(partial.clone());
        }
        out
    }
}


#[test]
fn composite_example() {
    use crate::alphabets::LATIN26;
    use crate::ciphers::{monoalphabetic::Caesar,transposition::Scytale};

    let c1 = Caesar::new(7, LATIN26);
    let c2 = Scytale::new(3);

    let composite = CompositeCipher::new(vec![&c1,&c2]);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGQ";
    let ciphertext = composite.encrypt(plaintext);
    let decrypt =    composite.decrypt(&ciphertext);

    assert_eq!(ciphertext,"AUYOMALVOXELBQSPBHJTGRWFIZKYVVVCNDLX");
    assert_eq!(decrypt,   "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGQ");

}
