use isl_rs as isl;

fn main() {
    let ctx = isl::Context::alloc();
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i, j]: 0<=i<1024 and 0<=j<512}");
    bset.dump();
    println!("Printed str is {}.", bset.to_str());
}
