use classic_crypto::ciphers::monoalphabetic::{Caesar,Substitution};
use classic_crypto::auxiliary::LATIN26;

fn main() {

    let plaintext = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
    println!("Our example plaintext is from Dickens:\n{}",plaintext);

    println!("\n\nMonoalphabetic substitution ciphers are perhaps the simplest kind of cipher and operating by replacing each symbol with another symbol. In a language with an ordered alphabet this can be done quickly by shifting each letter some amount, wrapping around back to the beginning if needed. This is known as the Caesar Cipher.");

    let caesar = Caesar::new(1, LATIN26);
    let ciphertext = caesar.encrypt(plaintext);
    println!("\n{}",ciphertext);

    println!("\n\nEven against an unskilled attack the Caesar Cipher can be broken just by testing all of the keys as few languages have more than a couple dozen letter symbols. This can be somewhat alleviated with the Affine Cipher which uses the rules of modular arithmetic to greatly increase the keyspace to include affine transformations over the alphabet. However, increasing the keyspace doesn't really improve the security of monoalphabetic substitution cipher. Consider the case where we match letters with arbitrary symbols. The keyspace is then the factorial of the number of letters in the alphabet.");

    let substitution = Substitution::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ","🌰🌱🌲🌳🌴🌵🌶️🌷🌸🌹🌺🌻🌼🌽🌾🌿🍀🍁🍂🍃🍄🍅🍆🍇🍈");
    let ciphertext = substitution.encrypt(plaintext);
    println!("\n{}",ciphertext);

    println!("\n\nDespite this huge apparent keyspace the structure of the text has not been obfuscated. The sequence 🌷🍂🍅🌰🍁 appears over and over, corresponding the phrase ITWAS from the plaintext. Even in a relatively short text enough patterns like this are apparent enough for an attacker to easily uravel the encryption.")
}