//! Simple Substitution Ciphers

pub mod caesar;
pub mod affine;
pub mod substitution;
pub mod decoder_ring;

pub use self::affine::Affine;
pub use self::caesar::Caesar;
pub use self::substitution::Substitution;
pub use self::decoder_ring::DecoderRing;