use isl_rs as isl;

#[test]
fn test_free_does_not_double_free() {
    let ctx = isl::Context::alloc();
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i, j]: 0<=i<=j<10}");
    bset.free();
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i, j]: 0<=i<=j<10}");
    assert_eq!(bset.dim(isl::DimType::Out), 2);
}

#[test]
fn test_isl_context_does_not_double_free() {
    let ctx = isl::Context::alloc();
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i, j]: 0<=i<=j<10}");
    let ctx1 = bset.get_ctx();
    assert_eq!(ctx1.ptr, ctx.ptr);
    bset.free();
    drop(ctx1);
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i, j]: 0<=i<=j<10}");
    assert_eq!(bset.dim(isl::DimType::Out), 2);
}
