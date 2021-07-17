#[cfg(test)]
mod monoalphabetic_tests {

    use crate::ciphers::monoalphabetic::{Atbash,Affine,Caesar,DecoderRing,Substitution};
    use crate::Cipher;
    use crate::alphabets::{LATIN26,LATIN26_QWERTY};
    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";

    #[test]
    fn substitution() {
        let substitution = Substitution::new(LATIN26, LATIN26_QWERTY);
    
        let ciphertext = substitution.encrypt(PLAINTEXT);
        let decrypted = substitution.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"ZITJXOEAWKGVFYGBOXDHLGCTKZITSQMNRGU");
        assert_eq!(decrypted,PLAINTEXT);
    }

    #[test]
    fn substitution_file() {
        let substitution = Substitution::new(LATIN26, LATIN26_QWERTY);

        substitution.encrypt_file("prepared_plaintext.txt", "substitution_ciphertext.txt").unwrap();
        substitution.decrypt_file("substitution_ciphertext.txt", "scratchpad.txt").unwrap();

    }

    #[test]
    fn atbash() {
        let atbash = Atbash::new(LATIN26_QWERTY);

        let ciphertext = atbash.encrypt(PLAINTEXT);
        let decrypted = atbash.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"CABMZLTOEVKNWDKYLZQJGKRBVCABIHUXFKS");
        assert_eq!(decrypted,PLAINTEXT);
    }

    #[test]
    fn affine() {
        let aff = Affine::new((1,3), LATIN26);
        let ciphertext = aff.encrypt(PLAINTEXT);
        let decrypted = aff.decrypt(&ciphertext);

        assert_eq!(ciphertext,"GWNXJZHFEARPOQRSZJLUDRMNAGWNIBYVKRT");
        assert_eq!(decrypted,PLAINTEXT);
    }

    #[test]
    fn caesar() {
        let caesar = Caesar::new(1, LATIN26);

        let ciphertext = caesar.encrypt(PLAINTEXT);
        let decrypted = caesar.decrypt(&ciphertext);

        assert_eq!(ciphertext,"UIFRVJDLCSPXOGPYJVNQTPWFSUIFMBAZEPH");
        assert_eq!(decrypted,PLAINTEXT);
    }

    #[test]
    fn caesar_file() {
        let caesar = Caesar::new(1, LATIN26);

        caesar.encrypt_file("prepared_plaintext.txt", "caesar_ciphertext.txt").unwrap();
        caesar.decrypt_file("caesar_ciphertext.txt", "scratchpad.txt").unwrap();

    }

    #[test]
    fn decoder_ring() {
        use crate::alphabets::GREEK24;

        let decoder  = DecoderRing::new(10,GREEK24);

        let plaintext = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ";

        let ciphertext = decoder.encrypt(plaintext);
        let decrypted = decoder.decrypt(&ciphertext);

        assert_eq!(&ciphertext,"10 11 12 13 14 15 16 17 18 19 20 21 22 23 0 1 2 3 4 5 6 7 8 9");
        assert_eq!(&decrypted,"ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ");
    }
    
    #[test]
    fn annie_ring() {
        let decoder  = DecoderRing::annie(5);

        let plaintext = "BESURETODRINKYOUROVALTINE";
        let ciphertext = decoder.encrypt(plaintext);
        let decrypted = decoder.decrypt(&ciphertext);

        assert_eq!(&ciphertext,"22 20 7 1 3 20 23 19 17 3 10 24 15 2 19 1 3 19 12 6 8 23 10 24 20");
        assert_eq!(&decrypted,"BESURETODRINKYOUROVALTINE");
    }
    
    #[test]
    fn midnight_ring() {
        let decoder  = DecoderRing::midnight(5);

        let plaintext = "SECRETSQUADRON";
        let ciphertext = decoder.encrypt(plaintext);
        let decrypted = decoder.decrypt(&ciphertext);

        assert_eq!(&ciphertext,"18 7 15 26 7 10 18 24 20 6 9 26 23 13");
        assert_eq!(&decrypted,"SECRETSQUADRON");
    }
}