fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--filter",
        "CoGetCallerTID",
        "IsCharLowerA",
        "SysFreeString",
        "IStringable",
        "GetLastError",
        "VhfStart",
        "RpcMgmtEnableIdleCleanup",
        "--flat",
        "--no-comment",
    ])
    .unwrap();
}
