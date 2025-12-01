use isl_rs as isl;

#[test]
fn test_free_does_not_double_free() {
    let ctx = isl::Context::alloc();
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i, j]: 0<=i<=j<10}").unwrap();
    bset.free().unwrap();
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i, j]: 0<=i<=j<10}").unwrap();
    assert_eq!(bset.dim(isl::DimType::Out).unwrap(), 2);
}

#[test]
fn test_isl_context_does_not_double_free() {
    let ctx = isl::Context::alloc();
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i, j]: 0<=i<=j<10}").unwrap();
    let ctx1 = bset.get_ctx();
    assert_eq!(ctx1.ptr, ctx.ptr);
    bset.free().unwrap();
    drop(ctx1);
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i, j]: 0<=i<=j<10}").unwrap();
    assert_eq!(bset.dim(isl::DimType::Out).unwrap(), 2);
}
