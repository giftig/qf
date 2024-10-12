use crate::args::SearchMode;
use crate::search::Hit;

/// Sort hits according to a range of criteria
///   - fewest leading spaces, as top-level definitions are more likely to be broadly relevant
///   - concrete method definitions first, according to the (naively-determined) presence of a body
///   - otherwise sort by filename and line number
pub fn sort_hits(hits: &mut Vec<Hit>, mode: &SearchMode) -> () {
    hits.sort_by_key(|h| {
        let mut leading_spaces = 0;

        for c in h.text.chars() {
            if c != ' ' || c != '\t' {
                break;
            }
            leading_spaces += 1;
        }

        let has_body = {
            *mode == SearchMode::Function &&
                h.text.chars().last().map(|c| vec!['{', ':', '='].contains(&c)).unwrap_or(false)
        };

        (leading_spaces, has_body, h.filename.clone(), h.line.clone().unwrap_or(0))
    });
}
