#[cfg(test)]
mod grille_tests {

    use crate::ciphers::grille::{Grille,TurningGrille};
    use crate::Cipher;
    use crate::alphabets::LATIN26;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";


    // Need to set the seed in order to get reproduceable results
    #[test]
    fn grille_random() {
        let gr = Grille::random(8, 9, 35, LATIN26);
        let ciphertext = gr.encrypt(PLAINTEXT);
        let decrypted = gr.decrypt(&ciphertext);

        //println!("{}", ciphertext);
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn grille_random_seeded() {
        let gr = Grille::random_seeded(8, 9, 35, LATIN26,10983426);
        //println!("{}",gr);
        let ciphertext = gr.encrypt(PLAINTEXT);
        let decrypted = gr.decrypt(&ciphertext);

        //println!("{}", ciphertext);
        assert_eq!(decrypted,PLAINTEXT)
    }

    #[test]
    fn turning_grille() {
        let key = vec![0,5,10,15,1,14,8,11,2,4,7,12,3,6,9,13];
        let gr = TurningGrille::new(4, key);

        //println!("{}",gr.display_grille_blank());
        let plaintext = "GETTINGEXACTLYSIXTYFOURLETTERSISAREALCHALLENGEWITHOUTSOMEPADDING";
        let ciphertext = gr.encrypt(plaintext);
        let decrypted = gr.decrypt(&ciphertext);
        //println!("{}",ciphertext);

        assert_eq!(decrypted,plaintext)
    }
}