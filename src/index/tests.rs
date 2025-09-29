use super::*;

use std::collections::HashMap;

use crate::index::models::ImportIndex;

#[test]
fn read_import_index_valid() {
    let f = "test/fixtures/import-index/valid.json".to_string();
    let actual = get_import_index("rust", &Some(f)).unwrap();
    let expected = ImportIndex {
        lang: "rust".to_string(),
        entries: HashMap::from([
            ("PathBuf".to_string(), "use std::path::PathBuf".to_string()),
            ("fs".to_string(), "use std::fs".to_string()),
        ]),
    };

    assert_eq!(actual, expected);
}

#[test]
fn read_import_index_bad_file() {
    let f = "test/fixtures/import-index/no-such-file.json".to_string();
    let actual = get_import_index("rust", &Some(f)).unwrap_err();

    assert!(matches!(actual, IndexError::IO { .. }))
}

#[test]
fn read_import_index_malformed_json() {
    let f = "test/fixtures/import-index/malformed.json".to_string();
    let actual = get_import_index("rust", &Some(f)).unwrap_err();

    assert!(matches!(actual, IndexError::Malformed { .. }))
}

#[test]
fn read_import_index_invalid_json() {
    let f = "test/fixtures/import-index/invalid.json".to_string();
    let actual = get_import_index("rust", &Some(f)).unwrap_err();

    assert!(matches!(actual, IndexError::Malformed { .. }))
}
