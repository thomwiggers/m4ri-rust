//! Links to m4ri/misc.h
use libc;

/// M4RI Internal representation
pub type Rci = libc::c_int;

/// M4RI Internal representation
pub type BIT = libc::c_int;

/// M4RI Internal representation
pub type Word = u64;

/// M4RI Internal representation
pub type Wi = libc::c_int;

/// The number of bits in a word
#[allow(non_upper_case_globals, dead_code)]
pub static m4ri_radix: libc::c_int = 64;

/// The number one as a word
#[allow(non_upper_case_globals, dead_code)]
pub static m4ri_one: Word = 1u64;

/// A word with all bits set
#[allow(non_upper_case_globals, dead_code)]
pub static m4ri_ffff: Word = 0xffffffffffffff;
