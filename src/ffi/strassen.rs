/// Links to strassen.h

use libc;
use ffi::mzd::Mzd;

#[link(name = "m4ri")]
extern "C" {

    /// \brief Matrix multiplication via the Strassen-Winograd matrix
    /// multiplication algorithm, i.e. compute C = AB.
    ///
    /// This is the wrapper function including bounds checks. See
    /// _mzd_mul_even for implementation details.
    ///
    /// \param C Preallocated product matrix, may be NULL for automatic creation.
    /// \param A Input matrix A
    /// \param B Input matrix B
    /// \param cutoff Minimal dimension for Strassen recursion.
    pub fn mzd_mul(c: *mut Mzd, a: *const Mzd, b: *const Mzd, cutoff: libc::c_int) -> *mut Mzd;

    /// \brief Matrix multiplication and in-place addition via the
    /// Strassen-Winograd matrix multiplication algorithm, i.e. compute
    /// C = C+ AB.
    ///
    /// This is the wrapper function including bounds checks. See
    /// _mzd_addmul_even for implementation details.
    ///
    /// \param C product matrix
    /// \param A Input matrix A
    /// \param B Input matrix B
    /// \param cutoff Minimal dimension for Strassen recursion.
    pub fn mzd_addmul(c: *mut Mzd, a: *const Mzd, b: *const Mzd, cutoff: libc::c_int) -> *mut Mzd;

}
