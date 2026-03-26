fn main() {
    println!("cargo:rerun-if-changed=../component/component.winmd");

    windows_bindgen::bindgen([
        "--in",
        "../component/component.winmd",
        &format!(
            "{}\\System32\\WinMetadata",
            std::env::var("WINDIR").unwrap()
        ),
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
