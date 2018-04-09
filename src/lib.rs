//! Fast binary matrix operations
//! Interfaces the M4RI library and provides friendly abstractions
extern crate libc;
#[cfg_attr(test, macro_use)]
extern crate vob;

pub mod ffi;
pub mod friendly;
