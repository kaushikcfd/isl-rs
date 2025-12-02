use isl_rs as isl;

fn main() {
    let ctx = isl::Context::alloc();
    let bset = isl::BasicSet::read_from_str(&ctx, "{[i] : 0<=i<N}");
    match &bset {
        Ok(b) => println!("BasicSet parsed: {}.", b.to_str().unwrap()),
        Err(e) => println!("Error in parsing: {}.", e),
    }
    println!("Done with the example!");
}
