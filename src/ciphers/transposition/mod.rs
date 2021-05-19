//! Transposition ciphers

pub mod scytale;
pub mod columnar;
pub mod route;
pub mod rail_fence;
pub mod columnar_incomplete;

mod transposition_tests;

pub use self::columnar::Columnar;
pub use self::columnar_incomplete::IncompleteColumnar;
pub use self::scytale::Scytale;
pub use self::rail_fence::RailFence;
pub use self::route::Route;