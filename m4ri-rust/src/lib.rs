#![cfg_attr(feature = "system_alloc", feature(alloc_system))]
//! Fast binary matrix operations
//! Interfaces the M4RI library and provides friendly abstractions
extern crate libc;
#[cfg_attr(test, macro_use)]
extern crate vob;

#[cfg(feature = "system_alloc")]
extern crate alloc_system;

extern crate rand;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
#[cfg(test)]
#[cfg(feature = "serde")]
extern crate serde_json;

extern crate m4ri_sys as ffi;

pub mod friendly;
