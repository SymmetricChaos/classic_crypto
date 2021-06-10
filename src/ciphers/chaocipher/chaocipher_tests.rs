
#[cfg(test)]
mod chaocipher_tests {

    use crate::ciphers::Chaocipher;

    #[test]
    fn test_chaociper() {
        let mut c = Chaocipher::new("HXUCZVAMDSLKPEFJRIGTWOBNYQ","PTLNBQDEOYSFAVZKGJRIHWXUMC");

        let plaintext = "WELLDONEISBETTERTHANWELLSAID";
        let encrypted = c.encrypt(plaintext);
        let decrypted = c.decrypt(&encrypted);

        assert_eq!(encrypted,"OAHQHCNYNXTSZJRRHJBYHQKSOUJY");
        assert_eq!(decrypted,plaintext)
    }
}