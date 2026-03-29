fn main() {
    windows_bindgen::bindgen([
        "--in",
        "../json_validator_winrt/sample.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Sample",
        "--flat",
    ])
    .unwrap();
}
