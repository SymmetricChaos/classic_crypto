#[cfg(test)]
mod vigenere_tests {
    use std::io::Error;

    use crate::ciphers::vigenere::{Autokey,Vigenere,RunningKey,ProgressiveKey};

    use crate::alphabets::LATIN26;

    #[test]
    fn vigenere() {
        use crate::Cipher;
        let vig = Vigenere::new("SECRET", LATIN26);
        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let ciphertext = vig.encrypt(plaintext);
        let decrypted = vig.decrypt(&ciphertext);
    
        println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
        
    }

    #[test]
    fn vigenere_autokey() {
        use crate::Cipher;
        let auto = Autokey::new("SECRET", LATIN26);
        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let ciphertext = auto.encrypt(plaintext);
        let decrypted = auto.decrypt(&ciphertext);

        println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
        
    }

    #[test]
    fn vigenere_running_key() -> Result<(),Error> {
        let auto = RunningKey::new("dickens.txt", LATIN26);
        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let ciphertext = auto.encrypt(plaintext)?;
        let decrypted = auto.decrypt(&ciphertext)?;
    
    
        println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
        
        Ok(())
    }

    #[test]
    fn vigenere_prog_key() {
        use crate::Cipher;
        let auto = ProgressiveKey::new("SECRET", 1,LATIN26);
        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let ciphertext = auto.encrypt(plaintext);
        let decrypted = auto.decrypt(&ciphertext);

        println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
        
    }

}