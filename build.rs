fn main() {
    cc::Build::new()
        .file("extern/picohttpparser/picohttpparser.c")
        .opt_level_str("fast")
        .flag("-funroll-loops")
        .flag("-flto=thin")
        .flag("-march=native")
        //.flag("-fwhole-program-vtables")
        //.flag("-mllvm")
        //.flag("--enable-loop-distribute")
        .compile("libpicohttpparser.a");
}
