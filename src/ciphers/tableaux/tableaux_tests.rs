#[cfg(test)]
mod porta_tests {

    use crate::ciphers::tableaux::{Porta,PortaProgressiveKey,Tableaux,TableauxAutokey,TableauxProgressiveKey};
    use crate::Cipher;
    use crate::alphabets::LATIN26;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";

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
    fn porta() {
        let porta = Porta::default("SECRET");
        let ciphertext = porta.encrypt(PLAINTEXT);
        let decrypted = porta.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"KWSIFRYZPJMAEUACYLVAEGGNIEVZNWDJRGV");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn porta_prog_key() {
        let porta = PortaProgressiveKey::default("SECRET",7);
        let ciphertext = porta.encrypt(PLAINTEXT);
        let decrypted = porta.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"KWSIFRSTWCFGDVMBZKQFJLLVGCXOPYHANKR");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn tableaux_random() {
        let tab = Tableaux::new("SECRET", TABLEAUX.to_vec(), LATIN26);
        let ciphertext = tab.encrypt(PLAINTEXT);
        let decrypted = tab.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TBUHXVJDJJUJWNHVZWKQFFRHGGQRPBQKKFT");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn tableaux_autokey_random() {
        let tab = TableauxAutokey::new("SECRET", TABLEAUX.to_vec(), LATIN26);
        let ciphertext = tab.encrypt(PLAINTEXT);
        let decrypted = tab.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TBUHXVYTLIGKVKYVSWAGYELKAGFGCSBTAUC");
        assert_eq!(decrypted,PLAINTEXT)
    }

    
    #[test]
    fn tableaux_prog_key_random() {
        let tab = TableauxProgressiveKey::new("SECRET", 7, TABLEAUX.to_vec(), LATIN26);
        let ciphertext = tab.encrypt(PLAINTEXT);
        let decrypted = tab.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TBUHXVGXEDSTXVKBHQAKIMCGRMBHKDIKUDO");
        assert_eq!(decrypted,PLAINTEXT)
    }

}