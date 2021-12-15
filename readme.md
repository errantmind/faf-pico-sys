# faf-pico-sys

Bindings to the C library `picohttpparser` optimized for maximum performance.

This was an internal crate used by a previous version of [FaF](https://github.com/errantmind/faf), but FaF has a faster way of parsing now. This crate is still a good option if you cannot use FaF's approach due to licensing incompatibility. It is very fast.


## License

This crate is licensed under the [Perl License](https://dev.perl.org/licenses/) or the [MIT License](https://opensource.org/licenses/MIT), at your option. This is the same as [picohttpparser](https://github.com/h2o/picohttpparser)


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
