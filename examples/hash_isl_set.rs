use isl_rs as isl;

fn main() {
    let ctx = isl::Context::alloc();
    let set = isl::Set::read_from_str(&ctx, "{[i, j]: 0<=i<1024 and 0<=j<512}");
    set.dump();
    println!("set.get_hash() is {}.", set.get_hash());
}
