#[cfg(test)]
mod polyalphabetic_tests {

    use crate::ciphers::polyalphabetic::{Vigenere,Beaufort,BeaufortVariant,Autokey};
    use crate::{Cipher};

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
        let vig = Vigenere::new("SECRET", LATIN26);
        let auto = Autokey::new(&vig);
        let ciphertext = auto.encrypt(PLAINTEXT);
        let decrypted = auto.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"LLGHYBVRFHIEPPPOXQZUGLEYDIZSGEQRKSR");
        assert_eq!(decrypted,PLAINTEXT)
    }

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
        let beau = Beaufort::new("SECRET", LATIN26);
        let auto = Autokey::new(&beau);
        let ciphertext = auto.encrypt(PLAINTEXT);
        let decrypted = auto.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"ZXYBKLRXDZGMPFNUFCBQWJOQVWLKKESVEQF");
        assert_eq!(decrypted,PLAINTEXT)
    }


    #[test]
    fn beaufort_var() {
        let beau_var = BeaufortVariant::new("SECRET", LATIN26);
        let ciphertext = beau_var.encrypt(PLAINTEXT);
        let decrypted = beau_var.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"BDCZQPKGZAKDVBMGFBULQXRLZPFNHHHUBXC");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn beaufort_var_autokey() {
        let beau_var = BeaufortVariant::new("SECRET", LATIN26);
        let auto = Autokey::new(&beau_var);
        let ciphertext = auto.encrypt(PLAINTEXT);
        let decrypted = auto.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"BDCZQPJDXBUOLVNGVYZKERMKFEPQQWIFWKV");
        assert_eq!(decrypted,PLAINTEXT)
    }


}