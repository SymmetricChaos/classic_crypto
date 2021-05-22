#[cfg(test)]
mod vigenere_tests {

    use crate::Code;
    use crate::alphabets::LATIN26;
    use crate::codes::godel::Godel;

    #[test]
    fn godel_test() {
        let godel = Godel::new(LATIN26);
        let coded = godel.encode("ATTACK");
        let decoded = godel.decode(&coded);

        //println!("{}",godel.char_map());

        assert_eq!(coded,"11104733683275566574453416633605957031250");
        assert_eq!(decoded,"ATTACK");
    }


}