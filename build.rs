fn main() {
    let mut builder = cc::Build::new();
    #[cfg(feature = "stack-clash-protection")]
    builder.flag_if_supported("-fstack-clash-protection");
    builder.flag_if_supported("-fsanitize=undefined");
    println!("cargo:rustc-link-arg=-fsanitize=undefined");
    (if option_env!("CC") == Some("clang") {
        builder.flag("-flto")
    } else {
        &mut builder
    })
    .file("alloca.c")
    .opt_level(2)
    .compile("calloca");
}
