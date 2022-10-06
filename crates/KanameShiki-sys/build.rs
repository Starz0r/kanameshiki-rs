use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rustc-link-lib=dylib=KanameShiki");

    let bindings_path_str = out_dir.join("bindings.rs");

    let bindings = bindgen::Builder::default()
        .header("src/kanameshiki.hpp")
        .clang_arg("--include-directory=libs/MemoryAllocator.KanameShiki/src/")
        .blacklist_function("std::.*")
        .clang_arg("-std=c++14")
        .use_core()
        .ctypes_prefix("cty")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join(bindings_path_str))
        .expect("Couldn't write bindings.");
}
