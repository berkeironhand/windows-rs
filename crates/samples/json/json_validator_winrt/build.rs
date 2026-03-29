fn main() {
    println!("cargo:rerun-if-changed=src/sample.rdl");

    windows_rdl::reader()
        .input("src/sample.rdl")
        .input("../../../libs/bindgen/default")
        .output("sample.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "sample.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Sample",
        "--flat",
        "--implement",
    ])
    .unwrap();
}
