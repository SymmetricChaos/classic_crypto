#[cfg(test)]
mod polyalphabetic_tests {

    use crate::ciphers::polyalphabetic::{Vigenere,Beaufort,BeaufortVariant,Autokey,ProgressiveKey,Tableaux};
    use crate::Cipher;

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
    fn vigenere_progkey() {
        let vig = Vigenere::new("SECRET", LATIN26);
        let prog = ProgressiveKey::new(&vig, 3);
        let ciphertext = prog.encrypt(PLAINTEXT);
        let decrypted = prog.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"LLGHYBXRGLVSLPWUTTNCDOIGVJVHBFGRUUZ");
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
    fn beaufort_progkey() {
        let beau = Beaufort::new("SECRET", LATIN26);
        let prog = ProgressiveKey::new(&beau, 3);
        let ciphertext = prog.encrypt(PLAINTEXT);
        let decrypted = prog.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"ZXYBKLTXEDTALFUABFPYTMSYNXHZFFIVOSN");
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

    #[test]
    fn beaufort_var_progkey() {
        let beau_var = BeaufortVariant::new("SECRET", LATIN26);
        let prog = ProgressiveKey::new(&beau_var, 3);
        let ciphertext = prog.encrypt(PLAINTEXT);
        let decrypted = prog.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"BDCZQPHDWXHAPVGAZVLCHOICNDTBVVSFMIN");
        assert_eq!(decrypted,PLAINTEXT)
    }


    // randomly generated tableaux
    const TABLEAUX: [&'static str; 26] = [
        "KBFOQMZYRNELUIVXGATWJHDPSC","QNMGTHDUZJAIKXVPFSELBCWYOR","IAKJQSXOCBDLMWYGHFUVENZPRT",
        "TAEDSLGVIJKRMYZPHOBUQXNFWC","IHEKSDTCRZYBXFMLPVAGONWUQJ","EXIBHKPJLZCGVDNFWTYMQOAURS",
        "YEMDIBZAPQLHTXWOJRKCFSVNUG","DORMVWNTGYCXQIFEUAZKLHJPBS","TOYNSJCIFUWPEDZMQKARVBXGLH",
        "RWALBDPXHTFVIYSJUNGKEMQZCO","UCKLWRGMVEFJQSTBPYXZDAONIH","PXGIAWHERUQMJFTZLYONDVCKSB",
        "RUXTMEYHCPDVONJKIBSQFLWZGA","MQSEAPDONIYVCJGXRHTFZBLKUW","DOXFPYEUQCVHLKBITZJGWMNRSA",
        "OPYNACTKJGHWMXZURVFIEBDSQL","KIWNXCMBRDOUTESFAQJZHGYVPL","YZKWFOSQLRPNUBMCDEAIGXVHJT",
        "DOUPSHRJECMXQAKIZVBTGFNYWL","FAGJPHMEQWDTLRXKVOZYSIUNCB","VKIJYQOPBAEFZNGULRTSHWDXCM",
        "VKLARZBIUTNHGEDMCXFOJPSWQY","RAGEWFNKLVOZSJMBHTQPICUDXY","PEWHORBVSMQKZUTJIGNYCLDXAF",
        "LBARIOTHCVMWUGFQPENSXJZKYD","SXVAFQCTBIPDWENYKUHJOLZGMR",
    ];

    #[test]
    fn tableaux() {
        let tab = Tableaux::new("SECRET", TABLEAUX.to_vec(), LATIN26);
        let ciphertext = tab.encrypt(PLAINTEXT);
        let decrypted = tab.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TBUHXVJDJJUJWNHVZWKQFFRHGGQRPBQKKFT");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn tableaux_auto() {
        let tab = Tableaux::new("SECRET", TABLEAUX.to_vec(), LATIN26);
        let auto = Autokey::new(&tab);
        let ciphertext = auto.encrypt(PLAINTEXT);
        let decrypted = auto.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TBUHXVYTLIGKVKYVSWAGYELKAGFGCSBTAUC");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn tableaux_prog() {
        let tab = Tableaux::new("SECRET", TABLEAUX.to_vec(), LATIN26);
        let prog = ProgressiveKey::new(&tab, 3);
        let ciphertext = prog.encrypt(PLAINTEXT);
        let decrypted = prog.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TBUHXVQTDRBESKBXLRCFYDLUIMLCZWSTQPC");
        assert_eq!(decrypted,PLAINTEXT)
    }
}