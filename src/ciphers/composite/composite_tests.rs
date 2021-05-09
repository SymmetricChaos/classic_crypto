#[cfg(test)]
mod composite_tests {

    use crate::ciphers::composite::{ADFGX,ADFGVX,Bifid,Nihilist};
    use crate::Cipher;
    use crate::alphabets::LATIN25_J;

    #[test]
    fn adfgx() {
        let adfgx = ADFGX::new("ELPEHANTS", "ZEBRAS");

        let plaintext = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOGX";
        let ciphertext = adfgx.encrypt(plaintext);
        let decrypted = adfgx.decrypt(&ciphertext);

        assert_eq!(ciphertext,"PCRNEKHIGANXSYPLMWUWBDTQHOCDUPBBCRMY");
        assert_eq!(decrypted, "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOGX");
    }

    
    #[test]
    fn adfgvx() {
        let adfgvx = ADFGVX::new("17ZEBRAS42", "ELEPHANTS");

        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
        let ciphertext = adfgvx.encrypt(plaintext);
        let decrypted = adfgvx.decrypt(&ciphertext);

        assert_eq!(ciphertext,"NNCZU1NRIOOQBXD2Z6AMQPL7GPTEXGVX0JNJ");
        assert_eq!(decrypted, "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX");
    }

    
    #[test]
    fn bifid() {
        let bifid = Bifid::new("ZEBRAS", "12345", 7, LATIN25_J);

        let plaintext = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
        let ciphertext = bifid.encrypt(plaintext);
        let decrypted = bifid.decrypt(&ciphertext);
        
        assert_eq!(ciphertext,"PRWGENCHRXDLDRTMLCOAHTZPECTEHAFFUWG");
        assert_eq!(decrypted,"THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG");
    }

    
    #[test]
    fn nihilist() {
        let nihilist = Nihilist::new("ZEBRAS", "ELEPHANT",LATIN25_J);
        
        let plaintext = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
        let ciphertext = nihilist.encrypt(plaintext);
        let decrypted = nihilist.decrypt(&ciphertext);

        assert_eq!(ciphertext,"57 65 24 87 82 47 63 78 25 48 54 96 72 39 83 99 44 85 47 86 52 57 93 57 26 79 43 55 65 30 52 100 35 76 37");
        assert_eq!(decrypted, "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG");
    }
}