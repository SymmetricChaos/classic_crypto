#[cfg(test)]
mod vigenere_tests {
    use std::io::Error;

    use crate::ciphers::beaufort::{Beaufort,Autokey,RunningKey,ProgressiveKey};
    use crate::Cipher;
    use crate::alphabets::LATIN26;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";

    #[test]
    fn beaufort() {
        let beau = Beaufort::new("SECRET", LATIN26);
        let ciphertext = beau.encrypt(PLAINTEXT);
        let decrypted = beau.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"ZXYBKLQUBAQXFZOUVZGPKDJPBLVNTTTGZDY");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn beaufort_autokey() {
        let beau = Autokey::new("SECRET", LATIN26);
        let ciphertext = beau.encrypt(PLAINTEXT);
        let decrypted = beau.decrypt(&ciphertext);

        assert_eq!(ciphertext,"ZXYBKLRXDZGMPFNUFCBQWJOQVWLKKESVEQF");
        assert_eq!(decrypted,PLAINTEXT)
    }

    
    #[test]
    fn beaufort_running_key() -> Result<(),Error> {
        let beau = RunningKey::new("dickens.txt", LATIN26);
        let ciphertext = beau.encrypt(PLAINTEXT)?;
        let decrypted = beau.decrypt(&ciphertext)?;

        assert_eq!(ciphertext,"PMSKYLFUANEXBAFLDKGTBIFOCOXSDRTVLRN");
        assert_eq!(decrypted,PLAINTEXT);

        Ok(())
    }

    #[test]
    fn beaufort_prog_key() {
        let beau = ProgressiveKey::new("SECRET", 1,LATIN26);
        let ciphertext = beau.encrypt(PLAINTEXT);
        let decrypted = beau.decrypt(&ciphertext);

        assert_eq!(ciphertext,"ZXYBKLRVCBRYHBQWXBJSNGMSFPZRXXYLEID");
        assert_eq!(decrypted,PLAINTEXT)
    }

}