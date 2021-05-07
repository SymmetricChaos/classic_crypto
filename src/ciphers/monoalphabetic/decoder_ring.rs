use std::fmt;
use rand::Rng;


/// The DecoderRing represents ciphers like those used on Little Orphan Annie and Captain Midnight. Letters are replaced with a number. Captain midnight indicated the key with an alphanumeric pair like A3 to indicate that the ring should be turned so at A lines up with 3. 
pub struct DecoderRing {
    index: usize,
    alphabet: String,
    length: usize,
}

impl DecoderRing {
    pub fn new(index: usize, alphabet: &str) -> DecoderRing {
        DecoderRing{ index, alphabet: alphabet.to_string(), length: alphabet.chars().count() }
    }

    pub fn annie(index: usize) -> DecoderRing {
        DecoderRing{ index, alphabet: "_ASLWIMVHFKXDPOEJBTNQZGUYRC".to_string(), length: 27 }
    }

    pub fn midnight(index: usize) -> DecoderRing {
        DecoderRing{ index, alphabet: "_AEXDTZKNYCJWSGUMBOQHRIVFPL".to_string(), length: 27 }
    }

}

impl crate::auxiliary::Cipher for DecoderRing {

    fn encrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = Vec::new();
        for s in symbols {
            let n = (self.alphabet.chars().position(|x| x == s).unwrap() + self.index) % self.length;
            out.push( format!("{}",n) )
        }
        out.join(" ")
    }

    fn decrypt(&self, text: &str) -> String {
        let nums = text.split(' ').map(|x| (x.parse::<usize>().unwrap() + self.length - self.index) % self.length );
        let mut out = String::new();
        for n in nums {
            out.push(self.alphabet.chars().nth(n).unwrap() )
        }
        out
    }

}

impl fmt::Display for DecoderRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decoder Ring Cipher\nkey: {}\nAlphabet: {}",self.index,self.alphabet)
    }
}

#[cfg(test)]
mod decoder_tests {
    use super::*;

    #[test]
    fn decoder_ring() {
        use crate::Cipher;
        use crate::alphabets::GREEK24;
        let decoder  = DecoderRing::new(10,GREEK24);
        let plaintext = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ";
        let ciphertext = decoder.encrypt(plaintext);
        let decrypted = decoder.decrypt(&ciphertext);
        assert_eq!(&ciphertext,"10 11 12 13 14 15 16 17 18 19 20 21 22 23 0 1 2 3 4 5 6 7 8 9");
        assert_eq!(&decrypted,"ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ");
    }
    
    #[test]
    fn annie_ring() {
        use crate::Cipher;
        let decoder  = DecoderRing::annie(5);
        let plaintext = "BESURETODRINKYOUROVALTINE";
        let ciphertext = decoder.encrypt(plaintext);
        let decrypted = decoder.decrypt(&ciphertext);
        assert_eq!(&ciphertext,"22 20 7 1 3 20 23 19 17 3 10 24 15 2 19 1 3 19 12 6 8 23 10 24 20");
        assert_eq!(&decrypted,"BESURETODRINKYOUROVALTINE");
    }
    
    #[test]
    fn midnight_ring() {
        use crate::Cipher;
        let decoder  = DecoderRing::midnight(5);
        let plaintext = "SECRETSQUADRON";
        let ciphertext = decoder.encrypt(plaintext);
        let decrypted = decoder.decrypt(&ciphertext);
        assert_eq!(&ciphertext,"18 7 15 26 7 10 18 24 20 6 9 26 23 13");
        assert_eq!(&decrypted,"SECRETSQUADRON");
    }
}


