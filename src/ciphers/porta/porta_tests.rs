#[cfg(test)]
mod porta_tests {

    use std::io::Error;

    use crate::ciphers::porta::{Porta,ProgressiveKey,RunningKey};
    use crate::Cipher;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";


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
        let porta = ProgressiveKey::default("SECRET",7);
        let ciphertext = porta.encrypt(PLAINTEXT);
        let decrypted = porta.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"KWSIFRSTWCFGDVMBZKQFJLLVGCXOPYHANKR");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn porta_running_key() -> Result<(),Error> {
        let auto = RunningKey::default("dickens.txt");
        let ciphertext = auto.encrypt(PLAINTEXT)?;
        let decrypted = auto.decrypt(&ciphertext)?;
    
        assert_eq!(ciphertext,"CQPDLRSZOCFAGUFGPFVLJDINIDWPSVDCXMP");
        assert_eq!(decrypted,PLAINTEXT);
        
        Ok(())
    }

}