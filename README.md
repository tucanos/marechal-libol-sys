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
cargo add --git https://github.com/jeromerobert/marechal-libol-sys.git
```
