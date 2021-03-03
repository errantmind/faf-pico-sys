fn main() {
    cc::Build::new()
        .file("extern/picohttpparser/picohttpparser.c")
        .opt_level_str(&"fast")
        .flag("-funroll-loops")
        .flag("-msse4")
        .flag("-flto=thin")
        .flag("-march=native")
        .compile("libpicohttpparser.a");
}
