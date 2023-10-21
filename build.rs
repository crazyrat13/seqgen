fn main() {
    // Copy the images to the output when generating documentation
    println!("cargo:rerun-if-changed=assets/doc");

    std::fs::copy("assets/seqgen_logo.svg", "target/doc/seqgen_logo.svg")
        .expect("Failed to copy crate logo when building documentation.");
}
