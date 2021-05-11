//! Transposition ciphers

pub mod scytale;
pub mod columnar;
pub mod route;
pub mod rail_fence;

mod transposition_tests;

pub use self::columnar::Columnar;
pub use self::scytale::Scytale;
pub use self::rail_fence::RailFence;