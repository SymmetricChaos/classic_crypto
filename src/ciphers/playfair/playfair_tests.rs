#[cfg(test)]
mod playfair_tests {

    use crate::{ciphers::{Playfair,TwoSquare,TwoSquareInverting,FourSquare,Slidefair}};
    use crate::Cipher;
    use crate::alphabets::{LATIN25_J,LATIN26};
    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOGX";

    #[test]
    fn playfair() {
        let playfair = Playfair::new("ZEBRAS", LATIN25_J, 5);
    
        let ciphertext = playfair.encrypt(PLAINTEXT);
        let decrypted = playfair.decrypt(&ciphertext);
    
        assert_eq!(ciphertext,"NMROVHDIRAPVQSQVHVKTCNECAQIZMRAUCPFY");
        assert_eq!(decrypted, "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOGX");
    }

    #[test]
    fn two_square() {
        let two_square = TwoSquare::new("EXAMPLE", "KEYWORD", LATIN25_J, 5);

        let ciphertext = &two_square.encrypt(PLAINTEXT);
        let decrypted = two_square.decrypt(ciphertext);

        assert_eq!(ciphertext,"NPOWGXARBREQNFUQWGSYMSXUWMXGLAPVMCUI");
        assert_eq!(decrypted, "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOGX");
    }

    #[test]
    fn two_square_inverting() {
        let two_square = TwoSquareInverting::new("EXAMPLE", "KEYWORD", LATIN25_J, 5);

        let ciphertext = &two_square.encrypt(PLAINTEXT);
        let decrypted = two_square.decrypt(ciphertext);

        assert_eq!(ciphertext,"PNWOXGRARBQEFNQUGWYSSMUXMWGXALVPCMIU");
        assert_eq!(decrypted, "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOGX");
    }

    #[test]
    fn four_square() {
        let four_square = FourSquare::new("EXAMPLE", "KEYWORD", LATIN25_J, 5);

        let ciphertext = &four_square.encrypt(PLAINTEXT);
        let decrypted = four_square.decrypt(ciphertext);

        assert_eq!(ciphertext,"RBESSCPAXNHXGAIXFQNGSHZKSNFYGKYZMICU");
        assert_eq!(decrypted, "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOGX");
    }

    #[test]
    fn slidefair() {
        let slide = Slidefair::new("ABCD", LATIN26);

        let ciphertext = &slide.encrypt("THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX");
        let decrypted = slide.decrypt(ciphertext);

        assert_eq!(ciphertext,"HTPFGWHFRBVPDPURUJONMUBYTRDIYNVCODWH");
        assert_eq!(decrypted, "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX");
    }
}