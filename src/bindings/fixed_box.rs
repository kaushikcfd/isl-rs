// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Context, Space};
use libc::uintptr_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Wraps `isl_fixed_box`.
pub struct FixedBox {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_fixed_box_get_ctx(box_: uintptr_t) -> uintptr_t;

    fn isl_fixed_box_get_space(box_: uintptr_t) -> uintptr_t;

    fn isl_fixed_box_is_valid(box_: uintptr_t) -> i32;

    fn isl_fixed_box_copy(box_: uintptr_t) -> uintptr_t;

    fn isl_fixed_box_free(box_: uintptr_t) -> uintptr_t;

    fn isl_fixed_box_to_str(box_: uintptr_t) -> *const c_char;

    fn isl_fixed_box_dump(box_: uintptr_t);

}

impl FixedBox {
    /// Wraps `isl_fixed_box_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let box_ = self;
        let box_ = box_.ptr;
        let isl_rs_result = unsafe { isl_fixed_box_get_ctx(box_) };
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_fixed_box_get_space`.
    pub fn get_space(&self) -> Space {
        let box_ = self;
        let box_ = box_.ptr;
        let isl_rs_result = unsafe { isl_fixed_box_get_space(box_) };
        let isl_rs_result = Space { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_fixed_box_is_valid`.
    pub fn is_valid(&self) -> bool {
        let box_ = self;
        let box_ = box_.ptr;
        let isl_rs_result = unsafe { isl_fixed_box_is_valid(box_) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_fixed_box_copy`.
    pub fn copy(&self) -> FixedBox {
        let box_ = self;
        let box_ = box_.ptr;
        let isl_rs_result = unsafe { isl_fixed_box_copy(box_) };
        let isl_rs_result = FixedBox { ptr: isl_rs_result,
                                       should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_fixed_box_free`.
    pub fn free(self) -> FixedBox {
        let box_ = self;
        let mut box_ = box_;
        box_.do_not_free_on_drop();
        let box_ = box_.ptr;
        let isl_rs_result = unsafe { isl_fixed_box_free(box_) };
        let isl_rs_result = FixedBox { ptr: isl_rs_result,
                                       should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_fixed_box_to_str`.
    pub fn to_str(&self) -> &str {
        let box_ = self;
        let box_ = box_.ptr;
        let isl_rs_result = unsafe { isl_fixed_box_to_str(box_) };
        let isl_rs_result = unsafe { CStr::from_ptr(isl_rs_result) };
        let isl_rs_result = isl_rs_result.to_str().unwrap();
        isl_rs_result
    }

    /// Wraps `isl_fixed_box_dump`.
    pub fn dump(&self) {
        let box_ = self;
        let box_ = box_.ptr;
        let isl_rs_result = unsafe { isl_fixed_box_dump(box_) };
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for FixedBox {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_fixed_box_free(self.ptr);
            }
        }
    }
}
