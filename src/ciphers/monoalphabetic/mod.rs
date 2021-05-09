//! Simple Substitution Ciphers

mod caesar;
mod affine;
mod substitution;
mod decoder_ring;
mod monoalphabetic_tests;

pub use self::affine::Affine;
pub use self::caesar::Caesar;
pub use self::substitution::{Substitution,Atbash};
pub use self::decoder_ring::DecoderRing;