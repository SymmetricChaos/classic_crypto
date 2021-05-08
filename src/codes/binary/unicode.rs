use std::{char, convert::TryInto};


pub fn bits_to_u32(text: &str) -> u32 {
    let mut out = 0u32;
    for (e, s) in text.chars().rev().enumerate() {
        match s {
            '0' => (),
            '1' => out += (2usize.pow(e.try_into().unwrap())) as u32,
            _ => panic!("bits can only be 0 or 1"),
        }
    }
    out
}



#[derive(Debug)]
pub struct UTF32 {}

impl UTF32 {

    pub fn default() -> UTF32 { UTF32{} }

    pub fn encode_bits(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(&format!("{:032b}", s as u32))
        }
        out
    }

    pub fn decode_bits(&self, text: &str) -> String {
        let mut out = String::new();
        let w = 32;
        for p in 0..(text.len()/w) {
            let group = &text[(p*w)..(p*w)+w];
            let n = bits_to_u32(group);
            out.push( char::from_u32(n).unwrap() )
        }
        out
    }

}


#[test]
fn unicode_utf32() {
    let utf32 = UTF32::default();
    let plaintext = "🌰🌱🌲🌳🌴🌵🌶️🌷🌸🌹🌺🌻🌼🌽🌾🌿🍀🍁🍂🍃🍄🍅🍆🍇🍈";
    let coded = utf32.encode_bits(plaintext);
    let decoded = utf32.decode_bits(&coded);

    assert_eq!(coded,"0000000000000001111100110011000000000000000000011111001100110001000000000000000111110011001100100000000000000001111100110011001100000000000000011111001100110100000000000000000111110011001101010000000000000001111100110011011000000000000000001111111000001111000000000000000111110011001101110000000000000001111100110011100000000000000000011111001100111001000000000000000111110011001110100000000000000001111100110011101100000000000000011111001100111100000000000000000111110011001111010000000000000001111100110011111000000000000000011111001100111111000000000000000111110011010000000000000000000001111100110100000100000000000000011111001101000010000000000000000111110011010000110000000000000001111100110100010000000000000000011111001101000101000000000000000111110011010001100000000000000001111100110100011100000000000000011111001101001000");
    assert_eq!(plaintext,decoded);
}
