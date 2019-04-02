//! Corresponds to `mp.h`
use crate::mzd::Mzd;
use libc;

extern "C" {
    /// Matrix multiplication via the cubic multiplication algorithm on multiple cores
    ///
    /// C: Preallocated product matrix, may be NULL for automatic creation
    /// A: Input matrix A
    /// B: Input matrix B
    /// cutoff: Minimal dimension for recursion
    pub fn mzd_mul_mp(c: *mut Mzd, a: *const Mzd, b: *const Mzd, cutoff: libc::c_int) -> *mut Mzd;

    /// Matrix multiplication and in-place additoin via th ecubic matrix multiplication
    /// algorithm on multiple cores. C = C + AB
    ///
    /// C: product matrix
    /// A: Input matrix
    /// B: Input matrix
    /// cutoff: Minimal dimension for recursion
    pub fn mzd_addmul_mp(
        c: *mut Mzd,
        a: *const Mzd,
        b: *const Mzd,
        cutoff: libc::c_int,
    ) -> *mut Mzd;
}
