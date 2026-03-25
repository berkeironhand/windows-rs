fn main() {
    println!("cargo:rerun-if-changed=../component/component.winmd");

    if !std::path::Path::new("../component/component.winmd").exists() {
        return;
    }

    windows_bindgen::bindgen([
        "--in",
        "../component/component.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_component",
        "--no-comment",
        "--flat",
        "--reference",
        "windows",
    ])
    .unwrap();
}
