#[cfg(test)]
mod porta_tests {

    use crate::ciphers::porta::{Porta};
    use crate::Cipher;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";

    #[test]
    fn porta() {
        let porta = Porta::default("SECRET");
        let ciphertext = porta.encrypt(PLAINTEXT);
        let decrypted = porta.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"KWSIFRYZPJMAEUACYLVAEGGNIEVZNWDJRGV");
        assert_eq!(decrypted,PLAINTEXT)
    }
}