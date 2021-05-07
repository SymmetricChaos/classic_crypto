#[derive(Debug)]
pub struct UTF32 {}

impl UTF32 {

    pub fn new() -> UTF32 { UTF32{} }

    pub fn encode_bits(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(&format!("{:032b}", s as u32))
        }
        out
    }

    pub fn decode_bits(&self, text: &str) -> String {
        let mut out = "".to_string();
        let w = 32;
        for p in 0..(text.len()/w) {
            let group = text[(p*w)..(p*w)+w].parse::<u8>().unwrap() as char;
            out.push( group )
        }
        out
    }
}

/* #[derive(Debug)]
pub struct UTF8 {}

impl UTF8 {

    pub fn new() -> UTF8 { UTF8{} }

    pub fn encode_bits(&self, text: &str) -> String {

    }

    pub fn decode_bits(&self, text: &str) -> String {

    }
} */


#[test]
fn unicode_utf32() {
    let utf32 = UTF32::new();
    let plaintext = "A";
    let coded = utf32.encode_bits(plaintext);
    //let decoded = Unicode.decode_bits(&coded);

    //println!("{}",Unicode);

    println!("{}",plaintext);
    println!("{}",coded);
    //println!("{}",decoded);
}
