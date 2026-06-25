use super::*;

// It's a fairly common pattern to use anonymous functions in consts in js, so we have to look
// for consts as well; unfortunately this does mean we might get false positives for normal consts.
const JS_FUNCTION_PATTERN: &str = r#"(?:function|const) (?:\(.+\) )?{}[\<\[\(: ]"#;

struct JsSmartMode {}

/// This is unfortunately copy-pasted from DefaultSmartMode except for using JS_FUNCTION_PATTERN
/// TODO: could be made a bit more DRY by having the SmartMode take the Strategy and reuse its
/// patterns instead.
impl SmartMode for JsSmartMode {
    /// First lowercase == function, uppercase == class
    fn get_pattern(&self, term: &str) -> String {
        if term.chars().next().map(|c| c.is_lowercase()).unwrap_or(true) {
            return JS_FUNCTION_PATTERN.to_owned();
        }

        CLASS_PATTERN.to_owned()
    }
}

pub(super) fn get_strategy() -> SearchStrategy {
    SearchStrategy::new(
        "{}",
        "{}",
        CLASS_PATTERN,
        JS_FUNCTION_PATTERN,
        IMPORT_PATTERN,
        Box::new(JsSmartMode {}),
    )
}
