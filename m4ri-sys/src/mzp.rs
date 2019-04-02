/// Corresponds to the mzp.h file.
///
/// Some functions are missing
use libc;

use crate::misc::Rci;
use crate::mzd::Mzd;

#[repr(C)]
pub struct Mzp {
    private: [u8; 0],
}

extern "C" {
    /// Construct an identity permutation
    ///
    /// length: the length of the permutation
    pub fn mzp_init(length: Rci) -> *mut Mzp;

    /// Free an Mzp
    pub fn mzp_free(p: *mut Mzp);

    /// Create a window into the permutation
    ///
    /// Use mzp_free_window to free the window
    pub fn mzp_init_window(p: *mut Mzp, begin: Rci, end: Rci);

    /// Free a permutation window created with Mzp_init_window
    pub fn Mzp_free_window(condemned: *mut Mzp);

    /// Copy permutation Q to P
    /// Target may be null
    pub fn mzp_copy(p: *mut Mzp, q: *const Mzp) -> *mut Mzp;

    /// Set the permutation to the identity permutation
    pub fn mzp_set_ui(p: *mut Mzp, value: libc::c_uint);

    /// Apply the permutation P to A from the left
    pub fn mzd_apply_p_left(a: *mut Mzd, p: *const Mzp);

    /// Apply the permutation P to A from the left, but transpose P
    pub fn mzd_apply_p_left_trans(a: *mut Mzd, p: *const Mzp);

    /// Apply the permutation P to A from the right
    pub fn mzd_apply_p_right(a: *mut Mzd, p: *const Mzp);

    /// Apply the permutation P to A from the right, but transpose P
    pub fn mzd_apply_p_right_trans(a: *mut Mzd, p: *const Mzp);

    /// Print the mzp
    pub fn mzp_print(p: *const Mzp);

// FIXME add missing components
}
