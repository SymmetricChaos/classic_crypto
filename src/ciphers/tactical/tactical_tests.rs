#[cfg(test)]
mod tactical_tests {

    use crate::ciphers::BATCO;

    const PLAINTEXT: &'static str = "01234567899876543210";

    #[test]
    fn test_batco() {
        let b = BATCO::random();
        b.set_key("2Z");

        println!("{}",b.key_row());
        println!("{}",b.encrypt("0123456789CH"))
    }
}