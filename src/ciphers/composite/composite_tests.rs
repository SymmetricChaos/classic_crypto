#[cfg(test)]
mod composite_tests {

    use crate::ciphers::composite::{ADFGX, ADFGVX, Bifid, Trifid, Nihilist, CompositeCipher};
    use crate::Cipher;
    use crate::alphabets::{LATIN25_J,LATIN26};

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
        assert_eq!(decrypted, "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG");
    }

    #[test]
    fn trifid() {
        let trifid = Trifid::new("RALPHEMERSON", "123", 7, "ABCDEFGHIJKLMNOPQRSTUVWXYZ#");

        let plaintext = "THEONLYWAYTOHAVEAFRIENDISTOBEONE";
        let ciphertext = trifid.encrypt(plaintext);
        let decrypted = trifid.decrypt(&ciphertext);
        
        assert_eq!(ciphertext,"QPTISKMUQPUBBWQBHPVWLFMBYQAJAEEU");
        assert_eq!(decrypted, "THEONLYWAYTOHAVEAFRIENDISTOBEONE");
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

    
    #[test]
    fn composite_example() {
        use crate::ciphers::{vigenere::Vigenere,transposition::Columnar};

        let c1 = Vigenere::new("APPLE", LATIN26);
        let c2 = Columnar::new("BANANA",LATIN26);

        let composite = CompositeCipher::new(vec![&c1,&c2]);
        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGQ";
        let ciphertext = composite.encrypt(plaintext);
        let decrypt =    composite.decrypt(&ciphertext);

        assert_eq!(ciphertext,"ORKRTFVTXVLTIDBJCQTJURTZLGQSOKWOQCQI");
        assert_eq!(decrypt,   "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGQ");
    }
}