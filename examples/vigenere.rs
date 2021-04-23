use classic_crypto::ciphers::{Caesar,Affine,Vigenere};
use classic_crypto::errors::CipherError;
use classic_crypto::alphabet::ALPHA26;

fn main() -> Result<(),CipherError> {

    let plaintext = "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair.";
    let decoded_nospace = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
    println!("Our example ciphertext is from Dickens and contains a lot of repetition\n{}",plaintext);

    let caesar = Caesar::new(1, ALPHA26.clone());
    println!("\n\n{}",caesar);
    let ciphertext = caesar.encode(plaintext)?;
    let cleartext = caesar.decode(&ciphertext)?;
    assert_eq!(cleartext,decoded_nospace);
    println!("{}",ciphertext);


    let affine = Affine::new((1,3), ALPHA26.clone());
    println!("\n\n{}",affine);
    let ciphertext = affine.encode(plaintext)?;
    let cleartext = affine.decode(&ciphertext)?;
    println!("{}",ciphertext);
    assert_eq!(cleartext,decoded_nospace);


    let vigenere = Vigenere::new("SECRET", ALPHA26.clone());
    println!("\n\n{}",vigenere);
    let ciphertext = vigenere.encode(plaintext)?;
    let cleartext = vigenere.decode(&ciphertext)?;
    println!("{}",ciphertext);
    assert_eq!(cleartext,decoded_nospace);


    Ok(())
}