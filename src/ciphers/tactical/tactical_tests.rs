#[cfg(test)]
mod tactical_tests {

    use crate::ciphers::{BATCO,DRYAD};
    use crate::Cipher;

    #[test]
    fn test_batco() {
        let b = BATCO::random();
        b.set_key("2Z");

        //println!("{}",b.code_page());

        let plaintext = "012CH345.6789CH";
        let ciphertext = b.encrypt(plaintext);
        let decrypted = b.decrypt(&ciphertext);

        //println!("{}",ciphertext);
        assert_eq!(plaintext,decrypted)
    }

    #[test]
    fn test_dryad() {
        let d = DRYAD::random();
        d.set_key('H');
        //println!("{}",d.code_page());

        let plaintext = "012345678";
        let ciphertext = d.encrypt(plaintext);
        let decrypted = d.decrypt(&ciphertext);

        //println!("{}",ciphertext);
        assert_eq!(plaintext,decrypted)
    }
}