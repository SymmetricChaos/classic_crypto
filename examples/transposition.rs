use classic_crypto::{alphabets::LATIN26, ciphers::transposition::{Columnar,Scytale}};
use classic_crypto::Cipher;

fn main() {

    println!("Transposition ciphers alter a message by making an anagram of the symbols. For these examples we will use the following short phrase:\nTHEQUICKBROWNFOXJUMPSOVERTHELAZYDOG");

    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";

    let columnar = Columnar::new("ZEBRAS",LATIN26);
    println!("\n{}",columnar);
    let ciphertext = columnar.encrypt(plaintext);
    let cleartext = columnar.decrypt(&ciphertext);
    println!("{}",ciphertext);
    assert_eq!(cleartext,"THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX");


    let scytale = Scytale::new(3);
    println!("\n{}",scytale);
    let ciphertext = scytale.encrypt(plaintext);
    let cleartext = scytale.decrypt(&ciphertext);
    println!("{}",ciphertext);
    assert_eq!(cleartext,"THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX");

}