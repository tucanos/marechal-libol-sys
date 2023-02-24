extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let include_dir = match env::var("LIBOL_INCLUDE_DIR") {
        Ok(x) => PathBuf::from(x),
        Err(_) => match env::var("LIBOL_DIR") {
            Ok(x) => PathBuf::from(x).join("include"),
            Err(_) => PathBuf::from("/usr/include"),
        },
    };
    let libol_h = include_dir
        .join("libol1.h")
        .into_os_string()
        .into_string()
        .expect("Not an UTF-8 path");
    let lib_dir = match env::var("LIBOL_LIB_DIR") {
        Ok(x) => Some(PathBuf::from(x)),
        Err(_) => env::var("LIBOL_DIR")
            .map(|x| PathBuf::from(x).join("lib"))
            .ok(),
    };
    if let Some(lib_dir) = lib_dir {
        let lib_dir = lib_dir
            .into_os_string()
            .into_string()
            .expect("Not an UTF-8 path");
        println!("cargo:rustc-link-search={}", lib_dir);
        #[cfg(target_os = "macos")]
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir);
        #[cfg(target_os = "linux")]
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir);
        // non standard key
        // see https://doc.rust-lang.org/cargo/reference/build-script-examples.html#linking-to-system-libraries
        // and https://github.com/rust-lang/cargo/issues/5077
        println!("cargo:rpath={}", lib_dir);
    }
    println!("cargo:rerun-if-changed={}", libol_h);
    println!("cargo:rustc-link-lib=OL.1");
    bindgen::Builder::default()
        .header(libol_h)
        .allowlist_type("TypTag")
        .rustified_enum("TypTag")
        .allowlist_function("Lol.*")
        .clang_args(
            env::var("LIBOL_CFLAGS")
                .unwrap_or_default()
                .split_whitespace(),
        )
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/binding.rs")
        .expect("Couldn't write bindings!");
}
