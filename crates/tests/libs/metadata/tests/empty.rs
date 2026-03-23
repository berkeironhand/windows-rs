use windows_metadata::*;

#[test]
fn test() {
    let reference = reader::TypeIndex::new(vec![]);
    let file = writer::File::new("test", &reference);
    let bytes = file.into_stream();
    std::fs::write("tests/empty.winmd", bytes).unwrap();

    let _index = reader::TypeIndex::read("tests/empty.winmd").unwrap();
}
