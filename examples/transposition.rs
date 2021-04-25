
use classic_crypto::ciphers::{Columnar};

fn main() {

    println!("Transposition ciphers alter a message by making an anagram of the symbols. For these examples we will use the following short phrase:\nTHEQUICKBROWNFOXJUMPSOVERTHELAZYDOG");

    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";

    let columnar = Columnar::new(vec![1,5,0,2,4,3]);
    println!("\nColumnar Cipher: {}",columnar);
    let ciphertext = columnar.encode(plaintext);
    let cleartext = columnar.decode(&ciphertext);
    println!("{}",ciphertext);
    assert_eq!(cleartext,"THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX");

}