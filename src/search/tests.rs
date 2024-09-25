use super::*;

use crate::args::SearchMode;

/// Prepend the prefix to the scala sample files for brevity
fn scala_file(s: &str) -> String {
    format!(
        "test/fixtures/scala/src/main/scala/com/xantoria/mmo/common/{}",
        s
    )
}

/// Prepend the prefix to the python sample files for brevity
fn py_file(s: &str) -> String {
    format!("test/fixtures/python/{}", s)
}

#[test]
/// Find definition of the Update trait. N.B. should specifically ignore substring matches like
/// InventoryUpdate and StatSheetUpdate present in the sample code
fn search_scala_trait() {
    let search = Search::new(&SearchMode::Class, &Language::Scala);
    let expected = vec![Hit {
        term: "Update".to_string(),
        filename: scala_file("model/updates/Update.scala"),
        line: Some(3),
        col: Some(1),
        text: "trait Update".to_string(),
        lang: DetectedLanguage::Scala,
    }];

    let actual = search.search("Update").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find definitions of the InventoryUpdate trait + companion object
fn search_scala_trait_with_companion() {
    let search = Search::new(&SearchMode::Class, &Language::Scala);
    let expected = vec![
        Hit {
            term: "InventoryUpdate".to_string(),
            filename: scala_file("model/updates/InventoryUpdate.scala"),
            line: Some(5),
            col: Some(8),
            text: "sealed trait InventoryUpdate extends Update {".to_string(),
            lang: DetectedLanguage::Scala,
        },
        Hit {
            term: "InventoryUpdate".to_string(),
            filename: scala_file("model/updates/InventoryUpdate.scala"),
            line: Some(11),
            col: Some(1),
            text: "object InventoryUpdate {".to_string(),
            lang: DetectedLanguage::Scala,
        },
    ];

    let actual = search.search("InventoryUpdate").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find a simple, single import
fn search_scala_import_single() {
    let search = Search::new(&SearchMode::Import, &Language::Scala);
    let expected = vec![Hit {
        term: "InventoryUpdate".to_string(),
        filename: scala_file("updates/inventory/InventoryUpdateResult.scala"),
        line: Some(5),
        col: Some(1),
        text: "import com.xantoria.mmo.common.model.updates.InventoryUpdate".to_string(),
        lang: DetectedLanguage::Scala,
    }];

    let actual = search.search("InventoryUpdate").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Make sure imports can be found in a multi-import
fn search_scala_import_multi() {
    let search = Search::new(&SearchMode::Import, &Language::Scala);
    let expected = vec![Hit {
        term: "Future".to_string(),
        filename: scala_file("model/updates/StatSheetUpdate.scala"),
        line: Some(4),
        col: Some(1),
        text: "import scala.concurrent.{ExecutionContext, Future}".to_string(),
        lang: DetectedLanguage::Scala,
    }];

    let actual = search.search("Future").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find definition of the toString def, which is overridden on one of the classes
fn search_scala_def() {
    let search = Search::new(&SearchMode::Function, &Language::Scala);
    let expected = vec![Hit {
        term: "toString".to_string(),
        filename: scala_file("model/updates/StatSheetUpdate.scala"),
        line: Some(25),
        col: Some(14),
        text: "    override def toString: String = {".to_string(),
        lang: DetectedLanguage::Scala,
    }];

    let actual = search.search("toString").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find a python class which extends Exception
fn search_python_class_extending_parent() {
    let search = Search::new(&SearchMode::Class, &Language::Python);
    let expected = vec![Hit {
        term: "TokeniserException".to_string(),
        filename: py_file("tokeniser.py"),
        line: Some(103),
        col: Some(1),
        text: "class TokeniserException(Exception):".to_string(),
        lang: DetectedLanguage::Python,
    }];

    let actual = search.search("TokeniserException").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find new-style python class which doesn't extend anything
/// N.B. also ensures it rules out classes with a substring of the term
fn search_python_class_plain() {
    let search = Search::new(&SearchMode::Class, &Language::Python);
    let expected = vec![Hit {
        term: "Cli".to_string(),
        filename: py_file("cli.py"),
        line: Some(17),
        col: Some(1),
        text: "class Cli:".to_string(),
        lang: DetectedLanguage::Python,
    }];

    let actual = search.search("Cli").unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn search_python_def() {
    let search = Search::new(&SearchMode::Function, &Language::Python);
    let expected = vec![Hit {
        term: "add_bookmark".to_string(),
        filename: py_file("cli.py"),
        line: Some(197),
        col: Some(5),
        text: "    def add_bookmark(self, name, path):".to_string(),
        lang: DetectedLanguage::Python,
    }];

    let actual = search.search("add_bookmark").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find a simple, single python import
fn search_python_import_single() {
    let search = Search::new(&SearchMode::Import, &Language::Python);
    let expected = vec![
        Hit {
            term: "readline".to_string(),
            filename: py_file("cli.py"),
            line: Some(6),
            col: Some(1),
            text: "import readline".to_string(),
            lang: DetectedLanguage::Python,
        },
        Hit {
            term: "readline".to_string(),
            filename: py_file("completion.py"),
            line: Some(3),
            col: Some(1),
            text: "import readline".to_string(),
            lang: DetectedLanguage::Python,
        },
    ];

    let actual = search.search("readline").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find a python import within a multi-import
fn search_python_import_multi() {
    let search = Search::new(&SearchMode::Import, &Language::Python);
    let expected = vec![Hit {
        term: "tokeniser".to_string(),
        filename: py_file("cli.py"),
        line: Some(11),
        col: Some(17),
        text: "from s3_browser import bookmarks, client, completion, paths, tokeniser, utils".to_string(),
        lang: DetectedLanguage::Python,
    }];

    let actual = search.search("tokeniser").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Make sure imports are found even if they've been renamed while importing
fn search_python_import_renamed() {
    let search = Search::new(&SearchMode::Import, &Language::Python);
    let expected = vec![Hit {
        term: "ArgumentParser".to_string(),
        filename: py_file("cli.py"),
        line: Some(12),
        col: Some(26),
        text: "from s3_browser.argparse import ArgumentParser as SafeParser".to_string(),
        lang: DetectedLanguage::Python,
    }];

    let actual = search.search("ArgumentParser").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Make sure the wrong language isn't searched when lang is provided
fn search_correct_lang_only() {
    let search = Search::new(&SearchMode::Function, &Language::Scala);
    let res = search.search("add_bookmark").unwrap();

    assert_eq!(res, vec![]);
}

#[test]
/// Find filenames which are a partial match for the search term
fn search_partial_filename() {
    let search = Search::new(&SearchMode::File, &Language::Auto);
    let expected = vec![
        Hit {
            term: "InventoryUpdate".to_string(),
            filename: scala_file("updates/inventory/InventoryUpdateResult.scala"),
            line: None,
            col: None,
            text: scala_file("updates/inventory/InventoryUpdateResult.scala"),
            lang: DetectedLanguage::Scala,
        },
        Hit {
            term: "InventoryUpdate".to_string(),
            filename: scala_file("model/updates/InventoryUpdate.scala"),
            line: None,
            col: None,
            text: scala_file("model/updates/InventoryUpdate.scala"),
            lang: DetectedLanguage::Scala,
        },
    ];

    let actual = search.search("InventoryUpdate").unwrap();

    assert_eq!(actual, expected);
}
