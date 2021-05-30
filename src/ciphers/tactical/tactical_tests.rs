#[cfg(test)]
mod tactical_tests {

    use crate::ciphers::{BATCO,DRYAD};
    use crate::Cipher;

    #[test]
    fn test_batco() {
        let b = BATCO::random_seeded(1066);
        //println!("{}",b.code_page());

        b.set_key("2Z");
        let plaintext = "012CH345.6789CH";
        let ciphertext = b.encrypt(plaintext);
        let decrypted = b.decrypt(&ciphertext);

        // Ciphertext involves random runtime choices so it is no consistent. We just check that it decodes.
        assert_eq!(plaintext,decrypted)
    }

    #[test]
    fn test_dryad() {
        let d = DRYAD::random_seeded(1776);
        //println!("{}",d.code_page());

        d.set_key('H');
        let plaintext = "0123456789";
        let ciphertext = d.encrypt(plaintext);
        let decrypted = d.decrypt(&ciphertext);

        // Ciphertext involves random runtime choices so it is no consistent. We just check that it decodes.
        assert_eq!(plaintext,decrypted)
    }
}