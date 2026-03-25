fn main() {
    let default = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../libs/bindgen/default");

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
        "test_composable",
        "--implement",
        "--no-comment",
        "--flat",
    ])
    .unwrap();
}
