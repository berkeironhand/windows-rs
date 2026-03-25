fn main() {
    println!("cargo:rerun-if-changed=../activation/metadata.winmd");

    if !std::path::Path::new("../activation/metadata.winmd").exists() {
        return;
    }

    windows_bindgen::bindgen([
        "--in",
        "../activation/metadata.winmd",
        "default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_activation",
        "--no-comment",
        "--flat",
    ])
    .unwrap();
}
