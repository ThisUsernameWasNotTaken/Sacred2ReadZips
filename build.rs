// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    // println!("cargo:rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.

    // cc::Build::new()
    //     .file("src/hello.c")
    //     .compile("hello");

    println!("yo");
    //../GrannyConverterLibrary/examples/converter/main.h
    cxx_build::bridge("src/building_bridges.rs") // returns a cc::Build
        .file("../GrannyConverterLibrary/examples/converter/main.cpp")
        .flag_if_supported("-std=c++11")
        .compile("cxxbridge-demo");
    // println!("cargo:rerun-if-changed=src/main.rs");
    // println!("cargo:rerun-if-changed=src/demo.cc");
    // println!("cargo:rerun-if-changed=include/demo.h");
}