#[test]
fn fn_abi() {
    windows_rdl::reader()
        .input("tests/fn_abi.rdl")
        .output("tests/fn_abi.winmd")
        .write()
        .unwrap();

    windows_rdl::writer()
        .input("tests/fn_abi.winmd")
        .output("tests/fn_abi_writer.rdl")
        .filter("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/fn_abi.winmd",
        "--out",
        "src/fn_abi.rs",
        "--filter",
        "Test",
        "--sys",
    ])
    .unwrap();
}

#[test]
fn fn_last_error() {
    windows_rdl::reader()
        .input("tests/fn_last_error.rdl")
        .output("tests/fn_last_error.winmd")
        .write()
        .unwrap();

    windows_rdl::writer()
        .input("tests/fn_last_error.winmd")
        .output("tests/fn_last_error.rdl")
        .filter("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/fn_last_error.winmd",
        "--out",
        "src/fn_last_error.rs",
        "--filter",
        "Test",
        "--sys",
    ])
    .unwrap();
}
