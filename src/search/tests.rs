use super::*;

use crate::args::SearchMode;

/// Prepend the prefix to the scala sample files for brevity
fn filename(s: &str) -> String {
    format!(
        "test/fixtures/scala/src/main/scala/com/xantoria/mmo/common/{}",
        s
    )
}

#[test]
/// Find definition of the Update trait. N.B. should specifically ignore substring matches like
/// InventoryUpdate and StatSheetUpdate present in the sample code
fn search_scala_trait() {
    let search = Search::new(&SearchMode::Class);
    let expected = vec![Hit {
        term: "Update".to_string(),
        filename: filename("model/updates/Update.scala"),
        line: Some(3),
        col: Some(1),
        text: "trait Update".to_string(),
    }];

    let actual = search.search("Update").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find definitions of the InventoryUpdate trait + companion object
fn search_scala_trait_with_companion() {
    let search = Search::new(&SearchMode::Class);
    let expected = vec![
        Hit {
            term: "InventoryUpdate".to_string(),
            filename: filename("model/updates/InventoryUpdate.scala"),
            line: Some(5),
            col: Some(8),
            text: "sealed trait InventoryUpdate extends Update {".to_string(),
        },
        Hit {
            term: "InventoryUpdate".to_string(),
            filename: filename("model/updates/InventoryUpdate.scala"),
            line: Some(11),
            col: Some(1),
            text: "object InventoryUpdate {".to_string(),
        },
    ];

    let actual = search.search("InventoryUpdate").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find definitions of the InventoryUpdate trait + companion object
fn search_scala_import_single() {
    let search = Search::new(&SearchMode::Import);
    let expected = vec![Hit {
        term: "InventoryUpdate".to_string(),
        filename: filename("updates/inventory/InventoryUpdateResult.scala"),
        line: Some(5),
        col: Some(1),
        text: "import com.xantoria.mmo.common.model.updates.InventoryUpdate".to_string(),
    }];

    let actual = search.search("InventoryUpdate").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find definitions of the InventoryUpdate trait + companion object
fn search_scala_import_multi() {
    let search = Search::new(&SearchMode::Import);
    let expected = vec![Hit {
        term: "Future".to_string(),
        filename: filename("model/updates/StatSheetUpdate.scala"),
        line: Some(4),
        col: Some(1),
        text: "import scala.concurrent.{ExecutionContext, Future}".to_string(),
    }];

    let actual = search.search("Future").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find definition of the toString def, which is overridden on one of the classes
fn search_scala_def() {
    let search = Search::new(&SearchMode::Function);
    let expected = vec![Hit {
        term: "toString".to_string(),
        filename: filename("model/updates/StatSheetUpdate.scala"),
        line: Some(25),
        col: Some(14),
        text: "    override def toString: String = {".to_string(),
    }];

    let actual = search.search("toString").unwrap();

    assert_eq!(actual, expected);
}

#[test]
/// Find filenames which are a partial match for the search term
fn search_partial_filename() {
    let search = Search::new(&SearchMode::File);
    let expected = vec![
        Hit {
            term: "InventoryUpdate".to_string(),
            filename: filename("updates/inventory/InventoryUpdateResult.scala"),
            line: None,
            col: None,
            text: filename("updates/inventory/InventoryUpdateResult.scala"),
        },
        Hit {
            term: "InventoryUpdate".to_string(),
            filename: filename("model/updates/InventoryUpdate.scala"),
            line: None,
            col: None,
            text: filename("model/updates/InventoryUpdate.scala"),
        },
    ];

    let actual = search.search("InventoryUpdate").unwrap();

    assert_eq!(actual, expected);
}
