fn main() {
    let default = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../libs/bindgen/default");

    println!("cargo:rerun-if-changed=src/component.rdl");

    windows_rdl::reader()
        .input("src/component.rdl")
        .reference(&format!("{default}/Windows.winmd"))
        .output("component.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "component.winmd",
        default,
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_component",
        "--flat",
        "--implement",
        "--no-comment",
        "--reference",
        "windows",
    ])
    .unwrap();
}
