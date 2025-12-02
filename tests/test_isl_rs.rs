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

#[test]
fn test_intersect_maps_with_different_spaces_errs() {
    let ctx = isl::Context::alloc();
    let bmap1 = isl::BasicMap::read_from_str(&ctx, "{[i, j] -> [2*i, 2*j] : 0<=i, j<10}").unwrap();
    let bmap2 = isl::BasicMap::read_from_str(&ctx, "{[i] -> [2*i] : 0<=i<10}").unwrap();
    let bmap3 = isl::BasicMap::read_from_str(&ctx, "{[i, j] -> [3*i, 3*j] : 0<=i, j<10}").unwrap();
    // bmap1 and bmap2 have different spaces should be an error.
    assert!(bmap1.copy().unwrap().intersect(bmap2).is_err());
    // bmap1 and bmap3 have same spaces should be Ok.
    assert!(bmap1.intersect(bmap3).is_ok());
}
