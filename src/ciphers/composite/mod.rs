//! Composite Ciphers

pub mod adfgvx;
pub mod adfgx;
pub mod nihilist;
pub mod vic;

pub use self::adfgx::ADFGX;
pub use self::adfgvx::ADFGVX;
pub use self::nihilist::{Nihilist,NihilistCyrillic};