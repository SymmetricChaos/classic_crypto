#[cfg(test)]
mod porta_tests {

    use crate::ciphers::bazeries::{Bazeries,BazeriesProgressive};
    use crate::Cipher;
    use crate::alphabets::LATIN36;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const WHEELS: [&'static str; 10] = ["VCYBUSFZJNI821KPAHEQX7WODT0RL3M694G5",
                                        "YAOUBC0KWX5HZ3R1IFG6PQDT2M7V8NLJ4S9E",
                                        "QXCV5L1BRIW8NFKEPHAM30U6YDG2SJ97Z4TO",
                                        "O0B8L9WMDYA2ZPENHT4K3RC5IJ6XU7SVFGQ1",
                                        "LKPBE23N5SR8DH4UT0QZIOGMVXAWJ16C7YF9",
                                        "K71Q9XLEBU4ASMDJWI2NH5GOFZCV83YTP0R6",
                                        "26FRG459UZVMNOXH7PTDLWAQ31YKC0J8ISBE",
                                        "CFNXVMDB7O0IEYH5G3K6J81AQRUT9W42LSPZ",
                                        "NFLA89JBHCY65P3ZM2KSVODXUQ14TIWG7ER0",
                                        "VPTZXBH0NESK89R26A7DIMCL5Q1G4UYOW3JF"];

    #[test]
    fn bazeries_random() {

        let baz = Bazeries::new(5, WHEELS.to_vec(), LATIN36);
        let ciphertext = baz.encrypt(PLAINTEXT);
        let decrypted = baz.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"MI38IGSA5DL3H0A4E2OHIKRKUKL3BC2C99W");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn bazeries_progressive_random() {

        let baz = BazeriesProgressive::new(5, 11, WHEELS.to_vec(), LATIN36);
        let ciphertext = baz.encrypt(PLAINTEXT);
        let decrypted = baz.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"MI38IGSA5DB2SZK5VD7A9N99KG5XINNO2BV");
        assert_eq!(decrypted,PLAINTEXT)
    }
    

}