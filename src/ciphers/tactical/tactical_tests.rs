#[cfg(test)]
mod tactical_tests {

    use crate::ciphers::BATCO;

    const PLAINTEXT: &'static str = "01234567899876543210";

    #[test]
    fn test_batco() {
        let b = BATCO::random();
        println!("{}",b.key_section());
        //println!("{}",b.cipher_section())
        println!("{}",b.key_to_row("2Z"))
    }
}