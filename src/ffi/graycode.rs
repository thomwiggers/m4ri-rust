/// Links to `graycode.h`

use libc;

#[repr(C)]
pub struct Code {
    /// Array of Gray code entries
    ord: *mut libc::c_int,
    /// Increment
    inc: *mut libc::c_int,
}

#[link(name = "m4ri")]
extern "C" {

    /// Returns the ith Gray code entry for a gray code of length 2^l
    ///
    /// i: index in the Gray code table
    /// l: Length of the Gray code
    ///
    /// Return: ith gray code entry
    pub fn m4ri_gray_code(i: libc::c_int, l: libc::c_int) -> libc::c_int;

    /// Fils var ord and var inc with Gray code dat afor a Gray
    /// code of length 2^l
    ///
    /// ord: Will hold gray code data, must be preallocated with correct size
    /// inc: Will hold some increment data, must be preallocated with correct size
    /// l: logarithm of the length of Gray code
    pub fn m4ri_build_code(ord: *mut libc::c_int, inc: *mut libc::c_int, l: libc::c_int);

    /// Generates global code book
    ///
    /// This function is called automatically when the shared library is loaded
    ///
    /// Not thread safe!
    pub fn m4ri_build_all_codes();

    /// Destroy global code book
    ///
    /// This function is called automatically when the shared library is unloaded
    ///
    /// Not thread safe
    pub fn m4ri_destroy_all_codes();

    /// Return the optimal var `k` for the given parameters
    ///
    /// If var c != 0, then var k for multiplication is returned,
    /// else var k for inversion. The optimal var k here means $0.75 log_2(n)$
    /// where `n` is `min(a,b)` for inversion and
    /// `b` for multiplication.
    ///
    /// a: Number of rows of (first) matrix
    /// b: Number of columns of (first) matrix
    /// c: Number of columns of second matrix (may be 0)
    ///
    /// Returns k
    pub fn m4ri_opt_k(a: libc::c_int, b: libc::c_int, c: libc::c_int) -> libc::c_int;
}
