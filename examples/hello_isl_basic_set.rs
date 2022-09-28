use isl_rs as isl;


fn main() {
    let ctx = isl::make_context();
    let bset = isl::get_basic_set_from_str(&ctx, "{[i]: 0<=i<1024}");
    isl::dump_basic_set(&bset);
}
