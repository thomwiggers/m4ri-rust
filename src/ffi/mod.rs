/// Implements the FFI to M4RI

mod brilliantrussian;
mod djb;
mod echelonform;
mod mzd;
mod mzp;
mod strassen;
mod mp;
mod graycode;
mod ple;

pub use self::mzd::*;
pub use self::mzp::*;
pub use self::strassen::*;
pub use self::mp::*;
pub use self::brilliantrussian::*;
pub use self::djb::*;
pub use self::echelonform::*;
pub use self::graycode::*;
pub use self::ple::*;
