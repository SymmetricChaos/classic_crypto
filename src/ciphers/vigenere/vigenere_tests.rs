#[cfg(test)]
mod vigenere_tests {
    use std::io::Error;

    use crate::ciphers::vigenere::{Autokey,Vigenere,RunningKey,ProgressiveKey};
    use crate::Cipher;

    use crate::alphabets::LATIN26;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";

    #[test]
    fn vigenere() {
        let vig = Vigenere::new("SECRET", LATIN26);
        let ciphertext = vig.encrypt(PLAINTEXT);
        let decrypted = vig.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"LLGHYBUODISPFJQONNETUFZXJXJVPTRCFFK");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn vigenere_autokey() {
        let auto = Autokey::new("SECRET", LATIN26);
        let ciphertext = auto.encrypt(PLAINTEXT);
        let decrypted = auto.decrypt(&ciphertext);

        assert_eq!(ciphertext,"LLGHYBVRFHIEPPPOXQZUGLEYDIZSGEQRKSR");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn vigenere_running_key() -> Result<(),Error> {
        let auto = RunningKey::new("dickens.txt", LATIN26);
        let ciphertext = auto.encrypt(PLAINTEXT)?;
        let decrypted = auto.decrypt(&ciphertext)?;
    
        assert_eq!(ciphertext,"BAAQMBJOCVGPBKHFVYEXLKVWKALAZRRRRTZ");
        assert_eq!(decrypted,PLAINTEXT);
        
        Ok(())
    }

    #[test]
    fn vigenere_prog_key() {
        let auto = ProgressiveKey::new("SECRET", 1,LATIN26);
        let ciphertext = auto.encrypt(PLAINTEXT);
        let decrypted = auto.decrypt(&ciphertext);

        assert_eq!(ciphertext,"LLGHYBVPEJTQHLSQPPHWXICANBNZTXWHKKP");
        assert_eq!(decrypted,PLAINTEXT)
    }

}