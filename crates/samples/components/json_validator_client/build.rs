fn main() {
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() != "msvc" {
        return;
    }

    println!("cargo:rerun-if-changed=src/client.cpp");
    println!("cargo:rustc-link-lib=onecoreuap");

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .file("src/client.cpp")
        .compile("client");
}
