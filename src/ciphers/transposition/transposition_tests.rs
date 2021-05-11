#[cfg(test)]
mod porta_tests {

    use crate::ciphers::transposition::{Columnar,RailFence,Route,Scytale};
    use crate::Cipher;
    use crate::alphabets::LATIN26;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";


    
    #[test]
    fn columnar() {
        let col = Columnar::new("FCBDAE", LATIN26);
        let ciphertext = col.encrypt(PLAINTEXT);
        let decrypted = col.decrypt(&ciphertext);

        assert_eq!(ciphertext,"UOJVLGEBOSHDHKFPTYQRXOEOIWUEAXTCNMRZ");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn rail_fence() {
    
        let railfence = RailFence::new(3);
        let ciphertext = railfence.encrypt(PLAINTEXT);
        let decrypted = railfence.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TUBNJSRLDHQIKRWFXUPOETEAYOXECOOMVHZG");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn route() {
        let route = Route::new((6,6),"");
        let ciphertext = route.encrypt(PLAINTEXT);
        //let decrypted = route.decrypt(&ciphertext);
    
        //assert_eq!(ciphertext,"TUBNJSRLDHQIKRWFXUPOETEAYOXECOOMVHZG");
        //assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn scytale() {
        let scytale = Scytale::new(3);
        let ciphertext = scytale.encrypt(PLAINTEXT);
        let decrypted = scytale.decrypt(&ciphertext);

        assert_eq!(ciphertext,"TNRHFTEOHQXEUJLIUACMZKPYBSDROOOVGWEX");
        assert_eq!(decrypted,PLAINTEXT)
    }

}