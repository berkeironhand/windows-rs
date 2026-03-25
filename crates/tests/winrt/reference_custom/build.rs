fn main() {
    let default = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../libs/bindgen/default");

    println!("cargo:rerun-if-changed=src/test.rdl");

    windows_rdl::reader()
        .input("src/test.rdl")
        .reference(&format!("{default}/Windows.winmd"))
        .output("test.winmd")
        .write()
        .unwrap();

    _ = windows_bindgen::bindgen([
        "--in",
        default,
        "test.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Test",
        "Vector2",
        "IVector",
        "IAsyncAction",
        "--implement",
        "--flat",
        "--no-comment",
        "--no-deps",
        "--reference",
        "windows,skip-root,Windows.Foundation.IStringable",
    ]);
}
