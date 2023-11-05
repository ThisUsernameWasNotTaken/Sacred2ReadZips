fn main() {
    cc::Build::new()
        .file("src/bar1.c")
        .file("src/bar2.c")
        .include("src/include")
        .compile("bar");

    println!("cargo:rerun-if-changed=src/main");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
