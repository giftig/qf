pub mod ag;
pub mod args;
pub mod fmt;
pub mod search;

use std::process;

use clap::Parser;

use args::Args;
use fmt::HitFormatter;
use search::Search;

fn main() {
    let args = Args::parse();

    if let Err(e) = args.validate() {
        eprintln!("{}", e);
        process::exit(1);
    }

    let search = Search::new(&args.mode, &args.lang);
    let hits = search.search(&args.term).unwrap();
    let formatter = HitFormatter::new(&args.output_style);

    for h in hits {
        println!("{}", formatter.write(&h).unwrap());

        if args.first_hit {
            break;
        }
    }
}
