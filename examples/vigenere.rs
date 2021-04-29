use classic_crypto::ciphers::vigenere::{Vigenere,Autokey};
use classic_crypto::alphabets::LATIN26;

fn main()  {

    let plaintext = "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair.";
    let decryptd_nospace = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
    println!("Our example ciphertext is from Dickens and contains a lot of repetition\n{}",plaintext);


    let vigenere = Vigenere::new("SECRET", LATIN26);
    println!("\n\n{}",vigenere);
    let ciphertext = vigenere.encrypt(plaintext);
    let cleartext = vigenere.decrypt(&ciphertext);
    println!("{}",ciphertext);
    assert_eq!(cleartext,decryptd_nospace);

/*     let autokey = Autokey::new("SECRET", LATIN26);
    println!("\n\n{}",autokey);
    let ciphertext = autokey.encrypt(plaintext)?;
    let cleartext = autokey.decrypt(&ciphertext)?;
    println!("{}",ciphertext);
    assert_eq!(cleartext,decryptd_nospace);
 */
}