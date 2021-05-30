#[cfg(test)]
mod grille_tests {

    use crate::ciphers::grille::{Grille,TurningGrille};
    use crate::Cipher;
    use crate::alphabets::LATIN26;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";


    // Need to set the seed in order to get reproduceable results
    #[test]
    fn grille() {
        let holes = vec![0, 3, 4, 7, 9, 12, 13, 14, 15, 18, 20, 25, 26, 29, 30, 32, 34, 36, 38, 41, 46, 47, 48, 49, 52, 55, 57, 61, 62, 64, 66, 67, 69, 70, 71];
        let gr = Grille::new((8,9), holes, LATIN26);
        let ciphertext = gr.encrypt(PLAINTEXT);
        let decrypted = gr.decrypt(&ciphertext);

        //println!("{}",gr.display_grille_blank());
        
        //println!("{}",gr.display_grille_encrypted(PLAINTEXT));

        //println!("{}", ciphertext);
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn turning_grille() {
        let key = vec![0,5,10,15,1,14,8,11,2,4,7,12,3,6,9,13];
        let gr = TurningGrille::new(4, key);

        println!("{}",gr.display_grille_blank());
    }
}