pub struct Context {
    ptr: u64
}

pub struct BasicSet {
    ptr: u64
}


extern "C" {
    // context
    fn isl_ctx_alloc() -> u64;
    fn isl_ctx_free(x: u64);

    // basic_set
    fn isl_basic_set_read_from_str(ctx: u64, code: &str) -> u64;
    fn isl_basic_set_dump(x: u64);
    fn isl_basic_set_free(x: u64);

}

// {{{ Context

pub fn make_context() -> Context {
    unsafe {
        Context { ptr: isl_ctx_alloc() }
    }
}


impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            isl_ctx_free(self.ptr)
        }
    }
}

// }}}


// {{{ BasicSet

pub fn get_basic_set_from_str(ctx: &Context, code: &str) -> BasicSet {
    unsafe {
        BasicSet {ptr: isl_basic_set_read_from_str(ctx.ptr, code)}
    }
}

pub fn dump_basic_set(bset: &BasicSet) {
    unsafe {
        isl_basic_set_dump(bset.ptr)
    }
}

impl Drop for BasicSet {
    fn drop(&mut self) {
        unsafe {
            isl_basic_set_free(self.ptr)
        }
    }
}

// }}}

// vim:fdm=marker
