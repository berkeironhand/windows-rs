fn main() {
    let default = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../libs/bindgen/default");

    println!("cargo:rerun-if-changed=src/test.rdl");

    windows_rdl::reader()
        .input("src/test.rdl")
        .reference(&format!("{default}/Windows.winmd"))
        .output("test.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "test.winmd",
        default,
        "--out",
        "src/bindings.rs",
        "--filter",
        "Test",
        "--implement",
        "--flat",
        "--no-comment",
        "--reference",
        "windows_collections,flat,Windows",
    ])
    .unwrap();

    #[cfg(target_env = "msvc")]
    {
        println!("cargo:rerun-if-changed=src/interop.cpp");
        println!("cargo:rustc-link-lib=onecoreuap");

        let windir = std::env::var("windir").unwrap();
        let winmetadata = format!("{windir}\\System32\\WinMetadata");
        let include = std::env::var("OUT_DIR").unwrap();

        cppwinrt::cppwinrt(["-in", "test.winmd", &winmetadata, "-out", &include]);

        cc::Build::new()
            .cpp(true)
            .std("c++20")
            .flag("/EHsc")
            .file("src/interop.cpp")
            .include(include)
            .compile("interop");
    }
}
