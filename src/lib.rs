use libc::uintptr_t;
use std::ffi::CString;
use std::os::raw::c_char;

pub struct Context {
    ptr: uintptr_t,
}

pub struct BasicSet {
    ptr: uintptr_t,
}

extern "C" {
    // context
    fn isl_ctx_alloc() -> uintptr_t;
    fn isl_ctx_free(x: uintptr_t);

    // basic_set
    fn isl_basic_set_read_from_str(ctx: uintptr_t, code: *const c_char) -> uintptr_t;
    fn isl_basic_set_dump(x: uintptr_t);
    fn isl_basic_set_free(x: uintptr_t);

}

// {{{ Context

pub fn make_context() -> Context {
    unsafe { Context { ptr: isl_ctx_alloc() } }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { isl_ctx_free(self.ptr) }
    }
}

// }}}

// {{{ BasicSet

pub fn get_basic_set_from_str(ctx: &Context, code: &str) -> BasicSet {
    let s = CString::new(code).expect("CString::new failed");
    unsafe { BasicSet { ptr: isl_basic_set_read_from_str(ctx.ptr, s.as_ptr()) } }
}

pub fn dump_basic_set(bset: &BasicSet) {
    unsafe { isl_basic_set_dump(bset.ptr) }
}

impl Drop for BasicSet {
    fn drop(&mut self) {
        unsafe { isl_basic_set_free(self.ptr) }
    }
}

// }}}

// vim:fdm=marker
