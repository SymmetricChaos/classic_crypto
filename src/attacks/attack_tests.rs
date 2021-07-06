// There are only 26 possibly keys for the Caesar cipher so trying them all is trivial. The challenge in automating the results in scoring the text.


#[cfg(test)]
mod attack_tests {
    use crate::ciphers::{
        transposition::Scytale,
        monoalphabetic::Caesar};
    use crate::Cipher;
    use crate::alphabets::{LATIN26};
    use crate::attacks::{caesar_attack,scytale_attack};

    #[test]
    fn attack_on_caesar() {
        let cipher = Caesar::new(5,LATIN26);
        let ciphertext = cipher.encrypt("THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG");
        println!("{:?}",caesar_attack(&ciphertext,LATIN26))
    }

    #[test]
    fn attack_on_scytale() {
        let cipher = Scytale::new(5);
        let ciphertext = cipher.encrypt("THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG");
        println!("{:?}",scytale_attack(&ciphertext))
    }
}