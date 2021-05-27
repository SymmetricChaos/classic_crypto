#[cfg(test)]
mod transposition_tests {

    use crate::ciphers::transposition::{Columnar, RailFence, Route, Scytale};
    use crate::Cipher;
    use crate::alphabets::LATIN26;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";

    #[test]
    fn columnar() {
        let col = Columnar::new("FCBDAE", LATIN26);
        let ciphertext = col.encrypt("THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG");
        let decrypted = col.decrypt(&ciphertext);

        assert_eq!(ciphertext,"IWUEAEBOSHDHKFPTYQRXOEOTCNMRZUOJVLG");
        assert_eq!(decrypted,"THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG")
    }

    #[test]
    fn columnar_complete() {
        let col = Columnar::new("FCBDAE", LATIN26);
        let ciphertext = col.encrypt(PLAINTEXT);
        let decrypted = col.decrypt(&ciphertext);

        assert_eq!(ciphertext,"IWUEAXEBOSHDHKFPTYQRXOEOTCNMRZUOJVLG");
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn rail_fence_stripe() {
        let railfence = RailFence::new(3);
        let ciphertext = railfence.encrypt(PLAINTEXT);
        let decrypted = railfence.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TUBNJSRLDHQIKRWFXUPOETEAYOXECOOMVHZG");
        assert_eq!(decrypted,PLAINTEXT)
    }
/*  #[test]
    fn route_stripe() {
        let route = Route::new((6,6),"stripe");
        let ciphertext = route.encrypt(PLAINTEXT);
        let decrypted = route.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TCNMRZHKFPTYEBOSHDQRXOEOUOJVLGIWUEAX");
        assert_eq!(decrypted,PLAINTEXT)
    } */

    #[test]
    fn route_snake() {
        let route = Route::new((6,6),"snake");
        let ciphertext = route.encrypt(PLAINTEXT);
        let decrypted = route.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"TCNMRZYTPFKHEBOSHDOEOXRQUOJVLGXAEUWI");
        assert_eq!(decrypted,PLAINTEXT)
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