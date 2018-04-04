/// Links to `echelonform.h`
use libc;

use ffi::misc::Rci;
use ffi::mzd::Mzd;

extern "C" {

    /// (Reduced) row echelon form
    ///
    /// A: matrix
    /// full: return the reduced row echelon form, not only upper
    /// triangular form.
    ///
    /// Return: Rank of A
    pub fn mzd_echelonize(a: *mut Mzd, full: libc::c_int) -> Rci;

    /// (Reduced) row echelon form using PLUQ factorisation
    ///
    /// A: matrix
    /// full: return the reduced row echelon form, not only upper
    /// triangular form.
    ///
    /// See `mzd_pluq()`
    ///
    /// Return: Rank of A
    pub fn mzd_echelonize_pluq(a: *mut Mzd, full: libc::c_int) -> Rci;

    /// Matrix elimination using the method of the four russians
    ///
    /// A: matrix to be reduced
    /// full: return the reduced row echelon form, not only upper triangular form
    /// k: M4RI parameter, set 0 for auto-choose
    ///
    /// Return: rank of A
    pub fn mzd_echelonize_m4ri(a: *mut Mzd, full: libc::c_int, k: libc::c_int) -> Rci;

}
