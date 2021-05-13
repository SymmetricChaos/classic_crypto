#[cfg(test)]
mod vigenere_tests {

    use crate::Code;
    use crate::alphabets::ASCII95;
    use crate::codes::binary::{ASCII,Bacon,Base64};

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";


    #[test]
    fn ascii8() {
        let ascii = ASCII::default8();
        let coded = ascii.encode(PLAINTEXT);
        let decoded = ascii.decode(&coded);

        assert_eq!(coded,"0101010001001000010001010101000101010101010010010100001101001011010000100101001001001111010101110100111001000110010011110101100001001010010101010100110101010000010100110100111101010110010001010101001001010100010010000100010101001100010000010101101001011001010001000100111101000111");
        assert_eq!(decoded,PLAINTEXT);
    }

    #[test]
    fn ascii7() {
        let ascii = ASCII::default7();
        let coded = ascii.encode(PLAINTEXT);
        let decoded = ascii.decode(&coded);

        assert_eq!(coded,"10101001001000100010110100011010101100100110000111001011100001010100101001111101011110011101000110100111110110001001010101010110011011010000101001110011111010110100010110100101010100100100010001011001100100000110110101011001100010010011111000111");
        assert_eq!(decoded,PLAINTEXT);
    }

    #[test]
    fn bacon_default() {
        let bacon = Bacon::default();
        let coded = bacon.encode(PLAINTEXT);
        let decoded = bacon.decode(&coded);

        println!("{}",coded);
        assert_eq!(decoded,PLAINTEXT);
    }

    #[test]
    fn bacon_ascii() {
        let bacon = Bacon::new(ASCII95);
        let plaintext = "The quick (BROWN) fox jumps over the [LAZY] dog!";
        let coded = bacon.encode(plaintext);
        let decoded = bacon.decode(&coded);

        println!("{}",coded);
        assert_eq!(decoded,plaintext);
    }

    #[test]
    fn base64_default() {
        let bacon = Base64::default();
        let coded = bacon.encode(PLAINTEXT);
        let decoded = bacon.decode(&coded);

        println!("{}",coded);
        assert_eq!(decoded,PLAINTEXT);
    }

}