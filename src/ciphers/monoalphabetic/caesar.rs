use std::fmt;

/// The Caesar Cipher substitutes symbols simply by shifting them some distance along the alphabet
pub struct Caesar {
    key: usize,
    alphabet: String,
    length: usize,
}

impl Caesar {
    pub fn new(key: usize, alphabet: &str) -> Caesar {
        Caesar{ key, alphabet: alphabet.to_string(), length: alphabet.chars().count() }
    }

    fn char_to_val(&self, c: char) -> usize {
        self.alphabet.chars().position(|x| x == c).unwrap()
    }

    fn val_to_char(&self, v: usize) -> char {
        self.alphabet.chars().nth(v).unwrap()
    }

    pub fn encrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = (self.char_to_val(s) + self.key) % self.length;
            out.push(self.val_to_char(n))
        }
        out
    }

    pub fn decrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = (self.char_to_val(s) + self.length - self.key) % self.length;
            out.push(self.val_to_char(n))
        }
        out
    }
}

impl fmt::Display for Caesar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caesar Cipher\nkey: {}",self.key)
    }
}




#[test]
fn caesar() {
    use crate::alphabets::LATIN26;
    let caesar  = Caesar::new(1, LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = caesar.encrypt(plaintext);
    let decrypted = caesar.decrypt(&ciphertext);
    assert_eq!(&ciphertext,"UIFRVJDLCSPXOGPYKVNQTPWFSUIFMBAZEPH");
    assert_eq!(&decrypted,"THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG");
    
}