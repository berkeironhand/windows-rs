fn main() {
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() != "msvc" {
        return;
    }

    println!("cargo:rerun-if-changed=src/test.idl");
    let metadata_dir = format!(
        "{}\\System32\\WinMetadata",
        std::env::var("WINDIR").unwrap()
    );
    let mut command = std::process::Command::new("midlrt.exe");
    println!("cargo:rerun-if-changed=src/interop.cpp");
    println!("cargo:rustc-link-lib=onecoreuap");

    command.args([
        "/winrt",
        "/nomidl",
        "/h",
        "nul",
        "/metadata_dir",
        &metadata_dir,
        "/reference",
        &format!("{metadata_dir}\\Windows.Foundation.winmd"),
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
        &metadata_dir,
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

    cppwinrt::cppwinrt(["-in", "test.winmd", &metadata_dir, "-out", &include]);

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .file("src/interop.cpp")
        .include(include)
        .compile("interop");
}
