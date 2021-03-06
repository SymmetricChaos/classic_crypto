#[cfg(test)]
mod polybius_tests {

    use crate::ciphers::Polybius;
    use crate::Cipher;
    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    use crate::alphabets::LATIN36;

    #[test]
    fn polybius() {
    
        let poly = Polybius::new("17ZEBRAS42",LATIN36,"123456", 2);
        
        //println!("{}",poly);

        let ciphertext = poly.encrypt(PLAINTEXT);
        let cleartext = poly.decrypt(&ciphertext);

        assert_eq!(cleartext,PLAINTEXT)
    }

    #[test]
    fn gen_polybius() {
    
        let poly = Polybius::new("ZEBRAS","ABCDEFGHIJKLMNOPQRSTUVWXYZ.","123", 3);

        //println!("{}",poly);

        let ciphertext = poly.encrypt(PLAINTEXT);
        let cleartext = poly.decrypt(&ciphertext);
    
        assert_eq!(cleartext,PLAINTEXT)
    }
}