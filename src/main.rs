mod ag;
mod args;
mod index;
mod fmt;
mod search;
mod sort;

use std::process;

use clap::Parser;

use crate::ag::Ag;
use crate::args::{Args, Language, OutputStyle, SearchMode};
use crate::index::get_import_index;
use crate::fmt::HitFormatter;
use crate::search::Search;

fn print_import_from_index(term: &str, lang: &Language, f: &Option<String>) {
    let lang_str = format!("{:?}", lang).to_lowercase();
    let index = get_import_index(&lang_str, &f).unwrap_or_else(|e| {
        eprintln!("Unexpected error reading index: {e}");
        process::exit(1);
    });

    for res in index.entries.get(term).into_iter() {
        println!("{res}");
    }
}

fn main() {
    let args = Args::parse();

    if let Err(e) = args.validate() {
        eprintln!("{}", e);
        process::exit(1);
    }

    let search = Search::new(Ag::default(), &args.mode, &args.lang);
    let hits = search.search(&args.term).unwrap();
    let formatter = HitFormatter::new(&args.output_style);

    for h in &hits {
        println!("{}", formatter.write(h).unwrap());

        if args.first_hit {
            break;
        }
    }

    // For generating imports specifically, we can make supplemental use of the imports index;
    // we'll only do this if we didn't find any hits in the local project
    if hits.len() == 0 &&
        args.use_import_index_file &&
        args.mode == SearchMode::Import &&
        args.output_style == OutputStyle::Import {

        // TODO: Also accept an override for the index file and pass it into the function
        print_import_from_index(&args.term, &args.lang, &args.import_index_file)
    }
}
