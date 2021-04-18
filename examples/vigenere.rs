use classic_crypto::vigenere::{Caesar,Vigenere,Autokey};
use classic_crypto::errors::CipherError;

fn main() -> Result<(),CipherError> {

    let plaintext = "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair.";
    let decoded_nospace = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
    println!("Our example ciphertext is from Dickens and contains a lot of repetition\n{}",plaintext);

    let caesar = Caesar::new(1);
    println!("\n\n{}",caesar);
    let ciphertext = caesar.encode(plaintext)?;
    let cleartext = caesar.decode(&ciphertext)?;
    assert_eq!(cleartext,decoded_nospace);
    println!("{}",ciphertext);


    let vigenere = Vigenere::new("SECRET".to_string().into_bytes());
    println!("\n\n{}",vigenere);
    let ciphertext = vigenere.encode(plaintext)?;
    let cleartext = vigenere.decode(&ciphertext)?;
    println!("{}",ciphertext);
    assert_eq!(cleartext,decoded_nospace);

    
    let autokey = Autokey::new("SECRET".to_string().into_bytes());
    println!("\n\n{}",autokey);
    let ciphertext = autokey.encode(plaintext)?;
    let cleartext = autokey.decode(&ciphertext)?;
    println!("{}",ciphertext);
    assert_eq!(cleartext,decoded_nospace);
    
    Ok(())
}