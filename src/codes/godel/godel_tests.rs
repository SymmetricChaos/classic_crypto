#[cfg(test)]
mod vigenere_tests {

    use crate::Code;
    use crate::alphabets::LATIN26;
    use crate::codes::godel::Godel;

    #[test]
    fn godel_test() {
        let ascii = Godel::new(LATIN26);
        let coded = ascii.encode("ATTACK");
        let decoded = ascii.decode(&coded);

        println!("{}",coded);

        //assert_eq!(coded,"");
        //assert_eq!(decoded,PLAINTEXT);
    }


}