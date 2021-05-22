#[cfg(test)]
mod binary_code_tests {

    use crate::Code;
    use crate::alphabets::{ASCII95,LATIN26_FREQ};
    use crate::codes::binary::{ASCII,Bacon,Base64,BaudotITA2,Fibonacci,MorseITU,UTF32,UTF8};

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
    fn unicode_utf32_bits() {
        let utf32 = UTF32::bits();
        let plaintext = "平仮名 -> ひらがな -> hiragana";
        let coded = utf32.encode(plaintext);
        let decoded = utf32.decode(&coded);

        assert_eq!(coded,"0000000000000000010111100111001100000000000000000100111011101110000000000000000001010100000011010000000000000000000000000010000000000000000000000000000000101101000000000000000000000000001111100000000000000000000000000010000000000000000000000011000001110010000000000000000000110000100010010000000000000000001100000100110000000000000000000011000001101010000000000000000000000000001000000000000000000000000000000010110100000000000000000000000000111110000000000000000000000000001000000000000000000000000000000110100000000000000000000000000001101001000000000000000000000000011100100000000000000000000000000110000100000000000000000000000001100111000000000000000000000000011000010000000000000000000000000110111000000000000000000000000001100001");
        assert_eq!(decoded,plaintext);
    }

    #[test]
    fn unicode_utf32_hex() {
        let utf32 = UTF32::hex();
        let plaintext = "平仮名 -> ひらがな -> hiragana";
        let coded = utf32.encode(plaintext);
        let decoded = utf32.decode(&coded);

        assert_eq!(coded,"00005e7300004eee0000540d000000200000002d0000003e0000002000003072000030890000304c0000306a000000200000002d0000003e000000200000006800000069000000720000006100000067000000610000006e00000061");
        assert_eq!(decoded,plaintext);
    }

    #[test]
    fn check_utf8() {
        let utf8 = UTF8::default();

        let plaintext = "平仮名 -> ひらがな -> hiragana";
        let encoded = utf8.encode(plaintext);
        let decoded = utf8.decode(&encoded);

        assert_eq!("11100101101110011011001111100100101110111010111011100101100100001000110100100000001011010011111000100000111000111000000110110010111000111000001010001001111000111000000110001100111000111000000110101010001000000010110100111110001000000110100001101001011100100110000101100111011000010110111001100001",encoded);
        assert_eq!(decoded,plaintext);
    }

}