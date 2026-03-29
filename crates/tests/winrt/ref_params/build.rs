fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=src/test.idl");
        println!("cargo:rerun-if-changed=src/interop.cpp");
        println!("cargo:rustc-link-lib=onecoreuap");

        let mut command = std::process::Command::new("midlrt.exe");

        command.args([
            "/winrt",
            "/nomidl",
            "/h",
            "nul",
            "/metadata_dir",
            "../../../libs/bindgen/default",
            "/reference",
            "../../../libs/bindgen/default/Windows.winmd",
            "/winmd",
            "test.winmd",
            "src/test.idl",
        ]);

        if !command.status().unwrap().success() {
            panic!("Failed to run midlrt");
        }

        windows_bindgen::bindgen([
            "--in",
            "test.winmd",
            "../../../libs/bindgen/default",
            "--out",
            "src/bindings.rs",
            "--filter",
            "Test",
            "--implement",
            "--flat",
            "--no-comment",
        ])
        .unwrap();

        let include = std::env::var("OUT_DIR").unwrap();

        cppwinrt::cppwinrt([
            "-in",
            "test.winmd",
            "../../../libs/bindgen/default",
            "-out",
            &include,
        ]);

        cc::Build::new()
            .cpp(true)
            .std("c++20")
            .flag("/EHsc")
            .file("src/interop.cpp")
            .include(include)
            .compile("interop");
    }
}
