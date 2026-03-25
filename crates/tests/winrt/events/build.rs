fn main() {
    let default = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../libs/bindgen/default");

    println!("cargo:rerun-if-changed=src/metadata.rdl");

    windows_rdl::reader()
        .input("src/metadata.rdl")
        .reference(&format!("{default}/Windows.winmd"))
        .output("metadata.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "metadata.winmd",
        default,
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_events",
        "--implement",
        "--no-comment",
        "--flat",
        "--reference",
        "windows",
    ])
    .unwrap();
}
