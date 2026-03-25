fn main() {
    println!("cargo:rerun-if-changed=../overloads/metadata.winmd");

    if !std::path::Path::new("../overloads/metadata.winmd").exists() {
        return;
    }

    windows_bindgen::bindgen([
        "--in",
        "../overloads/metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_overloads",
        "--no-comment",
        "--flat",
    ])
    .unwrap();
}
