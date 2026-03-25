fn main() {
    println!("cargo:rerun-if-changed=../json_validator_winrt/sample.winmd");

    if !std::path::Path::new("../json_validator_winrt/sample.winmd").exists() {
        return;
    }

    windows_bindgen::bindgen([
        "--in",
        "../json_validator_winrt/sample.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "--out",
        "src/bindings.rs",
        "--filter",
        "Sample",
        "--flat",
    ])
    .unwrap();
}
