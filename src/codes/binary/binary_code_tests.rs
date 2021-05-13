#[cfg(test)]
mod vigenere_tests {

    use crate::Code;
    use crate::alphabets::{ASCII95,LATIN26_FREQ};
    use crate::codes::binary::{ASCII,Bacon,Base64,BaudotITA2,Fibonacci,MorseITU,UTF32};

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

    #[test]
    fn baudot_ita2() {
        let ita2 = BaudotITA2::default();
        let plaintext = "THEQUICK␎12345␏BROWNFOX";
        let coded = ita2.encode(plaintext);
        let decoded = ita2.decode(&coded);

        assert_eq!(coded,"1000010100000011011100111001100111001111110111011110011000010101010000111111100101010110001001101100011011100011101");
        assert_eq!(decoded,plaintext);
    }
    
    #[test]
    fn fibonacci_code() {
        let fib = Fibonacci::new(LATIN26_FREQ);
        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let coded = fib.encode(plaintext);
        let decoded = fib.decode(&coded);

        assert_eq!(coded,"01100001111101000110000011000111010111000001101010111000111011010001110011001001110110010001101000011000001110000111001011010111011000000111110001101100001111001011001100010011000101101001110111010011");
        assert_eq!(decoded,plaintext);
    }

    #[test]
    fn morse_itu() {
        let itu = MorseITU::default();
        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let coded = itu.encode(plaintext);
        let decoded = itu.decode(&coded);

        assert_eq!(coded,"11100101010100100111011101011100101011100101001110101110100111010111001110101010010111010011101110111001011101110011101001010111010011101110111001110101011100101110111011100101011100111011100101110111010010101001110111011100101010111001001011101001110010101010010010111010100101110011101110101001110101110111001110101001110111011100111011101");
        assert_eq!(decoded,plaintext);
    }

    #[test]
    fn unicode_utf32() {
        let utf32 = UTF32::default();
        let plaintext = "🌰🌱🌲🌳🌴🌵🌶️🌷🌸🌹🌺🌻🌼🌽🌾🌿🍀🍁🍂🍃🍄🍅🍆🍇🍈";
        let coded = utf32.encode(plaintext);
        let decoded = utf32.decode(&coded);

        assert_eq!(coded,"0000000000000001111100110011000000000000000000011111001100110001000000000000000111110011001100100000000000000001111100110011001100000000000000011111001100110100000000000000000111110011001101010000000000000001111100110011011000000000000000001111111000001111000000000000000111110011001101110000000000000001111100110011100000000000000000011111001100111001000000000000000111110011001110100000000000000001111100110011101100000000000000011111001100111100000000000000000111110011001111010000000000000001111100110011111000000000000000011111001100111111000000000000000111110011010000000000000000000001111100110100000100000000000000011111001101000010000000000000000111110011010000110000000000000001111100110100010000000000000000011111001101000101000000000000000111110011010001100000000000000001111100110100011100000000000000011111001101001000");
        assert_eq!(plaintext,decoded);
    }

}