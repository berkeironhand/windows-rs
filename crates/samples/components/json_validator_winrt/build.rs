fn main() {
    println!("cargo:rerun-if-changed=src/sample.rdl");

    let windows_foundation = format!(
        "{}\\System32\\WinMetadata\\Windows.Foundation.winmd",
        std::env::var("WINDIR").unwrap()
    );

    windows_rdl::reader()
        .input("src/sample.rdl")
        .reference(&windows_foundation)
        .output("sample.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "sample.winmd",
        &windows_foundation,
        "--out",
        "src/bindings.rs",
        "--filter",
        "Sample",
        "--flat",
        "--implement",
    ])
    .unwrap();
}
