use classic_crypto::ciphers::{Vigenere,Autokey};
use classic_crypto::errors::CipherError;
use classic_crypto::alphabet::ALPHA26;

fn main() -> Result<(),CipherError> {

    let plaintext = "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair.";
    let decoded_nospace = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
    println!("Our example ciphertext is from Dickens and contains a lot of repetition\n{}",plaintext);


    let vigenere = Vigenere::new("SECRET", ALPHA26.clone());
    println!("\n\n{}",vigenere);
    let ciphertext = vigenere.encode(plaintext)?;
    let cleartext = vigenere.decode(&ciphertext)?;
    println!("{}",ciphertext);
    assert_eq!(cleartext,decoded_nospace);

    let autokey = Autokey::new("SECRET", ALPHA26.clone());
    println!("\n\n{}",autokey);
    let ciphertext = autokey.encode(plaintext)?;
    let cleartext = autokey.decode(&ciphertext)?;
    println!("{}",ciphertext);
    assert_eq!(cleartext,decoded_nospace);

    Ok(())
}