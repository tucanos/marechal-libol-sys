# About

This is a [bindgen](https://github.com/rust-lang/rust-bindgen) wrapper of [Loic Marechal's libOL](https://github.com/LoicMarechal/libOL.git).


# Using

If `libOL.1.so` is not in the standard location you should define required environment variables in
[`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html#env) (or define them in any other way):

```toml
[env]
LIBOL_INLCUDE_DIR="/path/to/libol-install/include"
# or
LIBOL_DIR="/path/to/libol-install"
# optionally
LIBOL_LIB_DIR="/path/to/lib"
```

Then

```
cargo add --git https://github.com/tucanos/marechal-libol-sys.git
```

To have `rpath` propagated to your executables add the following code to you `build.rs` (see this [bug report](https://github.com/rust-lang/cargo/issues/5077) for details).

```rust
if let Ok(rpath) = std::env::var("DEP_OL_1_RPATH") {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{rpath}");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{rpath}");
}
```
