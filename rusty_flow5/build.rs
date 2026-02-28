use std::path::PathBuf;
use cxx_build;

fn main() {
    // Build the C++ parts with CMake (expects a `cpp/` directory).
    // Ask CMake to export `compile_commands.json` so we can discover include
    // dirs automatically and forward them to `cxx_build`.
    let dst = cmake::Config::new("cpp")
        .profile("Release")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .build();

    let include_dir: PathBuf = dst.join("include");
    let lib_dir: PathBuf = dst.join("lib");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    // Link the library produced by CMake. The target here is `wrapper`.
    println!("cargo:rustc-link-lib=static=wrapper");

    let mut builder = cxx_build::bridge("src/lib.rs");
    if let Some(p) = include_dir.to_str() {
        builder.include(p);
    }
    // Also include the cpp source dir so local headers are visible
    builder.include("cpp");
    // If CMake produced a compile_commands.json, parse it and forward any
    // `-I` / `-isystem` include paths to cxx_build so the generated C++ glue
    // sees the same headers the CMake target used.
    let build_dir = dst.join("build");
    let compile_commands = build_dir.join("compile_commands.json");
    if compile_commands.exists() {
        if let Ok(s) = std::fs::read_to_string(&compile_commands) {
            for piece in s.split("-I") {
                // take chars until space, quote, or end
                let mut p = String::new();
                let rest = piece.trim_start();
                for c in rest.chars() {
                    if c == ' ' || c == '"' || c == '\'' || c == '\n' || c == '\r' || c == '\t' {
                        break;
                    }
                    p.push(c);
                }
                if p.is_empty() {
                    continue;
                }
                // Trim possible leading quotes and escape sequences from JSON
                let p = p.trim_matches(|ch| ch == '"' || ch == '\\' );
                let path = std::path::Path::new(p);
                if path.exists() {
                    if let Some(s) = path.to_str() {
                        builder.include(s);
                    }
                }
            }
            // also handle -isystem occurrences
            for piece in s.split("-isystem") {
                let mut p = String::new();
                let rest = piece.trim_start();
                for c in rest.chars() {
                    if c == ' ' || c == '"' || c == '\'' || c == '\n' || c == '\r' || c == '\t' {
                        break;
                    }
                    p.push(c);
                }
                if p.is_empty() {
                    continue;
                }
                let p = p.trim_matches(|ch| ch == '"' || ch == '\\' );
                let path = std::path::Path::new(p);
                if path.exists() {
                    if let Some(s) = path.to_str() {
                        builder.include(s);
                    }
                }
            }
        }
    }

    builder.flag_if_supported("-std=c++17");
    builder.compile("bridge");

    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp/wrapper.h");
    println!("cargo:rerun-if-changed=cpp/wrapper.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");
}