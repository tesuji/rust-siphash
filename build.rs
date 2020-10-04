fn main() {
    cc::Build::new()
        .file("ciphash.c")
        .flag_if_supported("-flto=thin")
        .static_flag(true)
        .compile("ciphash");
}
