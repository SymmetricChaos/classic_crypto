#[cfg(test)]
mod tactical_tests {

    use crate::ciphers::BATCO;

    #[test]
    fn test_batco() {
        let b = BATCO::random();
        b.set_key("2Z");

        let plaintext = "012345.6789CH";
        let ciphertext = b.encrypt(plaintext);
        let decrypted = b.decrypt(&ciphertext);

        println!("{}",ciphertext);
        assert_eq!(plaintext,decrypted)
    }
}