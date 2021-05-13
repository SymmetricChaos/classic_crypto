#[cfg(test)]
mod porta_tests {

    use std::io::Error;

    use crate::ciphers::tableaux::{Tableaux,Autokey,ProgressiveKey,RunningKey};
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
    fn tableaux_random() {
        let tab = Tableaux::new("SECRET", TABLEAUX.to_vec(), LATIN26);
        let ciphertext = tab.encrypt(PLAINTEXT);
        let decrypted = tab.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TBUHXVJDJJUJWNHVZWKQFFRHGGQRPBQKKFT");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn tableaux_autokey_random() {
        let tab = Autokey::new("SECRET", TABLEAUX.to_vec(), LATIN26);
        let ciphertext = tab.encrypt(PLAINTEXT);
        let decrypted = tab.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TBUHXVYTLIGKVKYVSWAGYELKAGFGCSBTAUC");
        assert_eq!(decrypted,PLAINTEXT)
    }

    
    #[test]
    fn tableaux_prog_key_random() {
        let tab = ProgressiveKey::new("SECRET", 7, TABLEAUX.to_vec(), LATIN26);
        let ciphertext = tab.encrypt(PLAINTEXT);
        let decrypted = tab.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TBUHXVGXEDSTXVKBHQAKIMCGRMBHKDIKUDO");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn tableaux_running_key_random() -> Result<(),Error> {
        let auto = RunningKey::new("dickens.txt", TABLEAUX.to_vec(), LATIN26);
        let ciphertext = auto.encrypt(PLAINTEXT)?;
        let decrypted = auto.decrypt(&ciphertext)?;
    
        assert_eq!(ciphertext,"AFDECVKDUIBJWPRWOXKLUKOINHBDMSQTAVC");
        assert_eq!(decrypted,PLAINTEXT);
        
        Ok(())
    }

}