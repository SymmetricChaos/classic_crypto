#[cfg(test)]
mod vigenere_tests {

    use crate::Code;
    use crate::alphabets::ASCII95;
    use crate::codes::binary::{Bacon,Base64};

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";

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