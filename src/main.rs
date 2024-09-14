pub mod args;

use clap::Parser;

use args::Args;

fn main() {
    let args = Args::parse();
    println!("{:?}", &args);
}
