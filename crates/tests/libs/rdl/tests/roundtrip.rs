use std::sync::Arc;
use windows_metadata::reader::{File, TypeIndex};
use windows_rdl::*;

fn is_winmd_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("winmd"))
        .unwrap_or(false)
}

#[test]
fn roundtrip() {
    let mut paths: Vec<_> = std::fs::read_dir("tests/roundtrip")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rdl"))
        .collect();
    paths.sort();

    // Build the reference TypeIndex once so that all per-file iterations share it
    // rather than re-parsing the large winmd files on every call.
    let reference_dir = "../../../libs/bindgen/default";
    let reference_files: Vec<_> = std::fs::read_dir(reference_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| is_winmd_file(p))
        .filter_map(|p| File::read(&p))
        .collect();
    let reference = Arc::new(TypeIndex::new(reference_files));

    for path in &paths {
        let winmd = path.with_extension("winmd");

        reader()
            .input(path.to_str().unwrap())
            .reference_index(Arc::clone(&reference))
            .output(winmd.to_str().unwrap())
            .write()
            .unwrap();

        writer()
            .input(winmd.to_str().unwrap())
            .input(reference_dir)
            .output(path.to_str().unwrap())
            .filter("Test")
            .write()
            .unwrap();
    }
}
