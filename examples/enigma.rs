
use std::io::Error;

use classic_crypto::ciphers::enigma::{EnigmaM3,prep_file,ROTOR,REFLECTOR};

fn main() -> Result<(),Error> {

    prep_file("sample_text.txt","enigma_plaintext.txt")?;

    let plugs = "EJ OY IV AQ KW FX MT PS LU BD";
    let rotor1 = ROTOR["IV"].clone();
    let rotor2 = ROTOR["II"].clone();
    let rotor3 = ROTOR["V"].clone();
    let rotors = (rotor1, rotor2, rotor3);
    let reflector = REFLECTOR["B"].clone();
    let ring_positions = (14,22,25);

    let mut s = EnigmaM3::new( plugs, rotors, reflector, ring_positions );

    s.encrypt_file("enigma_plaintext.txt", "engima_ciphertext.txt")?;

    Ok(())
}