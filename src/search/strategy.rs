mod go;

use crate::args::{Language, SearchMode};

// This one is fairly complex due to different language patterns for imports:
//   - After "import" or "use" we match everything up to one of [., {:/] to try to find the last
//     separator and extract the symbol from the import. That's because scala/python/rust can use
//     brace / commas for multi import blocks, rust uses :: as a separator, and go uses
//     / as a separator. The space is because some imports will be simple and just a single term.
//   - We then match on the literal symbol which will be quoted and replaced in {}, like \Qfoo\E
//   - Finally we match and discard a character which can end the symbol: again a space, comma or
//     close brace for scala/python/rust, a semicolon for rust (though also possible in python
//     or scala) and a quotation mark for go.
//
// N.B. if this gets any more complex then most likely it should be broken into individual regexes
// for specific languages, especially as reqirements of one language may break those of another
// language. In that case the language should be detected first, and then the right regex applied.
const IMPORT_PATTERN: &str = r#"(?:import|use).*[\.\{{,:/" ]{}(?:[\{{\}},;/" ]|$)"#;

const CLASS_PATTERN: &str = {
    r#"(?:case class|class|trait|object|type|struct|impl|enum) {}\h*(?:[\[\(\{{: ]|$)"#
};

// N.B. the second optional block is unique to go struct methods
const FUNCTION_PATTERN: &str = r#"(?:def|fn|function|func) (?:\(.+\) )?{}[\<\[\(: ]"#;

trait SmartMode {
    /// A pattern to use to "smartfind" a symbol as either a class or function
    /// For most languages we search with a func pattern if it starts with a lowercase and a class
    /// pattern if it starts with uppercase, but some languages may have different patterns (go!)
    fn get_pattern(&self, term: &str) -> String;
}
pub struct DefaultSmartMode {}

impl SmartMode for DefaultSmartMode {
    /// First lowercase == function, uppercase == class
    fn get_pattern(&self, term: &str) -> String {
        if term.chars().next().map(|c| c.is_lowercase()).unwrap_or(true) {
            return FUNCTION_PATTERN.to_owned();
        }

        CLASS_PATTERN.to_owned()
    }
}

pub struct SearchStrategy {
    all_usage_pattern: String,
    file_pattern: String,
    class_pattern: String,
    function_pattern: String,
    import_pattern: String,
    smart_mode: Box<dyn SmartMode>,
}

impl SearchStrategy {
    fn new(
        all_usage: &str,
        file: &str,
        class: &str,
        function: &str,
        import: &str,
        smart: Box<dyn SmartMode>,
    ) -> SearchStrategy {
        SearchStrategy {
            all_usage_pattern: all_usage.to_owned(),
            file_pattern: file.to_owned(),
            class_pattern: class.to_owned(),
            function_pattern: function.to_owned(),
            import_pattern: import.to_owned(),
            smart_mode: smart,
        }
    }

    fn default() -> SearchStrategy {
        SearchStrategy::new(
            "{}",
            "{}",
            CLASS_PATTERN,
            FUNCTION_PATTERN,
            IMPORT_PATTERN,
            Box::new(DefaultSmartMode {}),
        )
    }

    /// Wrap the term in an appropriate regex depending on the search mode
    pub fn get_pattern(&self, mode: &SearchMode, term: &str) -> String {
        // Regex raw quote
        let raw = format!("\\Q{}\\E", term);

        let fmt = match *mode {
            SearchMode::AllUsage => &self.all_usage_pattern,
            SearchMode::Class => &self.class_pattern,
            SearchMode::File => &self.file_pattern,
            SearchMode::Function => &self.function_pattern,
            SearchMode::Import => &self.import_pattern,
            SearchMode::Smart => &self.smart_mode.get_pattern(term),
        };

        fmt.replace("{}", &raw)
    }
}

pub fn get_strategy(lang: &Language) -> SearchStrategy {
    match *lang {
        Language::Go => go::get_strategy(),
        _ => SearchStrategy::default(),
    }
}
