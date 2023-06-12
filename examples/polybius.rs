// use classic_crypto::ciphers::Polybius;
// use classic_crypto::ciphers::transposition::Columnar;
// use classic_crypto::Cipher;

// fn main()  {

//     let nospace = "ITWASTHEBESTOFTIMESITWASTHEWORSTOFTIMESITWASTHEAGEOFWISDOMITWASTHEAGEOFFOOLISHNESSITWASTHEEPOCHOFBELIEFITWASTHEEPOCHOFINCREDULITYITWASTHESEASONOFLIGHTITWASTHESEASONOFDARKNESSITWASTHESPRINGOFHOPEITWASTHEWINTEROFDESPAIR";
//     println!("Our example ciphertext is from Dickens and contains a lot of repetition\n{}",nospace);

//     println!("
// The Polybius Square dates back to Ancient Greece where it was originally proposed as a way to send singals. The alphabet it written into a grid with
// the sides labeled. Each symbol is then mapped to a unique pair of numbers. Below is an example Polybius square on a 6x6 grid with the Latin alphabet 
// and Indo-Arabic numerals.");

//     let polybius = Polybius::new("IHAVE17ZEBRASAND293HORSES", "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", "123456");
//     println!("\n\n{}",polybius);

//     println!("
// The letter 'A' would encrypt as the pair (1,3) while 'D' would encrypt as (3,1). When used in encryption these pairs are generally read off as 13 and 31.

// On its own the Polybius Square provides no more encryption than a simple substitution cipher but can form the basis of much stronger ciphers when 
// combined with other techniques. For example The ADFGVX cipher is a surprisingly effective cipher that combines the Polybius Square with Columnar 
// Transposition. This allows a single letter of plaintext to alter multiple letters of the ciphertext. This cipher takes its name from the symbols
// chosen by the German mmilitary to label the sides. Keying the square with the phrase `VICTORJAGT12BOXKAMPFER` we get:");

//     let polybius = Polybius::new("VICTORJAGT12BOXKAMPFER", "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", "ADFGVX");
//     println!("\n{}",polybius);

//     println!("We then encrypt a message in the usual way using the Polybius Square. So the message `ANGRIFFIMMORGENGRAUEN` becomes:\n");

//     println!("{}",polybius.encrypt("ANGRIFFIMMORGENGRAUEN"));

//     println!("Using the secret key CHIFRE")
// }