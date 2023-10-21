fn main() {
    // Copy the images to the output when generating documentation
    println!("cargo:rerun-if-changed=assets/doc");

    let res = std::env::current_dir();
    match res {
        Ok(path) => println!("{}", path.into_os_string().into_string().unwrap()),
        Err(_) => (),
    };

    std::fs::copy("assets/seqgen_logo.svg", "target/doc/seqgen_logo.svg")
        .expect("Failed to copy crate logo when building documentation.");
}
