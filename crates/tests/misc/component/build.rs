fn main() {
    println!("cargo:rerun-if-changed=src/component.rdl");

    windows_rdl::reader()
        .input("src/component.rdl")
        .input("../../../libs/bindgen/default")
        .output("component.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "component.winmd",
        "../../../libs/bindgen/default",
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
