fn main() {
    // Copy the images to the output when generating documentation
    println!("cargo:rerun-if-changed=assets/doc");

    let out_dir = match std::env::var("OUT_DIR") {
        Ok(val) => val,
        Err(_) => "target/doc".to_owned(),
    };

    std::fs::copy(
        "assets/seqgen_logo.svg",
        format!("{out_dir}/seqgen_logo.svg"),
    )
    .expect("Failed to copy crate logo when building documentation.");
}
