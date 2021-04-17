use classic_crypto::vigenere::{Caesar,Vigenere};
use classic_crypto::errors::CipherError;

fn main() -> Result<(),CipherError> {
    println!("\nCaesar Cipher");
    let mut caesar = Caesar::new(1);
    caesar.set_whitespace(true);
    let plaintext = "The quick brown fox jumped over the lazy dog.";
    let ciphertext = caesar.encode(plaintext)?;
    let cleartext = caesar.decode(&ciphertext)?;

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);

    
    println!("\nVigenere Cipher");
    let mut vigenere = Vigenere::new(vec![1,2,3]);
    vigenere.set_whitespace(true);
    let plaintext = "The quick brown fox jumped over the lazy dog.";
    let ciphertext = vigenere.encode(plaintext)?;
    let cleartext = vigenere.decode(&ciphertext)?;

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
    
    Ok(())
}