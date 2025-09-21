use super::*;

struct GoSmartMode {}

impl SmartMode for GoSmartMode {
    fn get_pattern(&self, _term: &str) -> String {
        return format!("(?:{CLASS_PATTERN}|{FUNCTION_PATTERN})");
    }
}

pub(super) fn get_strategy() -> SearchStrategy {
    SearchStrategy::new(
        "{}",
        "{}",
        CLASS_PATTERN,
        FUNCTION_PATTERN,
        IMPORT_PATTERN,
        Box::new(GoSmartMode {}),
    )
}
