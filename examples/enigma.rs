
use std::io::Error;

use classic_crypto::ciphers::{Enigma,Plugboard,prep_file,ENIGMA_ROTORS,ENIGMA_REFLECTORS};

fn main() -> Result<(),Error> {

    prep_file("sample_text.txt","enigma_plaintext.txt")?;

    let plugboard = Plugboard::new("EJ OY IV AQ KW FX MT PS LU BD");
    let rotor1 = ENIGMA_ROTORS["IV"].clone();
    let rotor2 = ENIGMA_ROTORS["II"].clone();
    let rotor3 = ENIGMA_ROTORS["V"].clone();
    let rotors = (rotor1, rotor2, rotor3);
    let reflector = ENIGMA_REFLECTORS["B"].clone();
    let ring_positions = (14,22,25);

    let mut s = Enigma::new( plugboard, rotors, reflector, ring_positions );

    s.encode_file("enigma_plaintext.txt", "engima_ciphertext.txt", (0,0,0))?;

    Ok(())
}