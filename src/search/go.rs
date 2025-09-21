
pub struct GoSearchStrategy {}

impl GoSearchStrategy {
    fn get_smart_pattern(term: &str) -> String {
        return format!("(?:{CLASS_PATTERN}|{FUNCTION_PATTERN})");
    }
}
