//! Fast binary matrix operations
//! Interfaces the M4RI library and provides friendly abstractions
extern crate libc;
//somehow the macro_use things have problems
#[allow(unused_imports)]
#[macro_use]
extern crate vob;

mod binary_matrix;
mod binary_vector;
pub mod ffi;

/// Friendly interfaces on the M4RI constructs
pub mod friendly {
    pub use super::binary_matrix::*;
    pub use super::binary_vector::*;
}
