use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;
use windows_rdl::*;

const INPUT: &str = r#"
#[win32]
mod Test {
    struct Color {
        R: u8,
        G: u8,
        B: u8,
    }
    struct Point {
        X: i32,
        Y: i32,
    }
}
"#;

fn input_winmd() -> &'static std::path::Path {
    static WINMD: OnceLock<std::path::PathBuf> = OnceLock::new();
    WINMD.get_or_init(|| {
        let path = std::env::temp_dir().join("windows_rdl_filter_input.winmd");
        reader()
            .input_str(INPUT)
            .output(&path.to_string_lossy())
            .write()
            .unwrap();
        path
    })
}

fn filter(filters: &[&str], expected: &str) {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let rdl = std::env::temp_dir().join(format!("windows_rdl_filter_{id}.rdl"));

    let winmd = input_winmd();
    let mut w = writer();
    w.input(&winmd.to_string_lossy())
        .output(&rdl.to_string_lossy());
    for f in filters {
        w.filter(f);
    }
    w.write().unwrap();

    let actual = std::fs::read_to_string(&rdl).unwrap();
    assert_eq!(actual.trim(), expected.trim());
}

#[test]
fn filter_namespace() {
    filter(
        &["Test"],
        r#"
#[win32]
mod Test {
    struct Color {
        R: u8,
        G: u8,
        B: u8,
    }
    struct Point {
        X: i32,
        Y: i32,
    }
}
"#,
    );
}

#[test]
fn filter_unqualified_type() {
    filter(
        &["Color"],
        r#"
#[win32]
mod Test {
    struct Color {
        R: u8,
        G: u8,
        B: u8,
    }
}
"#,
    );
}

#[test]
fn filter_qualified_type() {
    filter(
        &["Test.Point"],
        r#"
#[win32]
mod Test {
    struct Point {
        X: i32,
        Y: i32,
    }
}
"#,
    );
}

#[test]
fn filter_multiple_types() {
    filter(
        &["Test.Color", "Test.Point"],
        r#"
#[win32]
mod Test {
    struct Color {
        R: u8,
        G: u8,
        B: u8,
    }
    struct Point {
        X: i32,
        Y: i32,
    }
}
"#,
    );
}

#[test]
fn filter_exclude_type() {
    filter(
        &["Test", "!Test.Color"],
        r#"
#[win32]
mod Test {
    struct Point {
        X: i32,
        Y: i32,
    }
}
"#,
    );
}
