fn main() {
    println!("cargo:rerun-if-changed=../json_validator_winrt/sample.winmd");

    windows_bindgen::bindgen([
        "--in",
        "../json_validator_winrt/sample.winmd",
        &format!(
            "{}\\System32\\WinMetadata",
            std::env::var("WINDIR").unwrap()
        ),
        "--out",
        "src/bindings.rs",
        "--filter",
        "Sample",
        "--flat",
    ])
    .unwrap();
}
