fn main() {
    // Re-run build script when relevant files change
    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp/wrapper.h");
    println!("cargo:rerun-if-changed=cpp/wrapper.cpp");
    println!("cargo:rerun-if-changed=src/api/globals.rs");

    // Generate C++ files
    let _builder = cxx_build::bridge("src/api/globals.rs");

    // Configure and build the C++ project under `cpp/` using the cmake crate.
    // Request shared libraries so wrapper links to its dependencies and symbols
    let dst = cmake::Config::new("cpp")
        .define("BUILD_SHARED_LIBS", "ON")
        .build();

    // Tell cargo to link the produced shared library (libwrapper.so)
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=wrapper");
}
