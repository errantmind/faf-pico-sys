# faf-pico-sys

Bindings to the C library `picohttpparser`. This is an internal crate used by `faf`

## Notes
* Must install and compile with clang, gcc will fail
* See [the issue](https://github.com/seanmonstar/httparse/issues/85#issuecomment-787563457) which describes how to use this crate as a dependency

## References

https://doc.rust-lang.org/rustc/linker-plugin-lto.html

https://doc.rust-lang.org/reference/linkage.html

https://blog.llvm.org/2019/09/closing-gap-cross-language-lto-between.html

https://apt.llvm.org/

https://rustc-dev-guide.rust-lang.org/backend/updating-llvm.html

https://doc.rust-lang.org/nomicon/ffi.html
