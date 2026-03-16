use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/nested-enum.rdl")
        .output("tests/nested-enum.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/nested-enum.winmd")
        .output("tests/nested-enum.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
