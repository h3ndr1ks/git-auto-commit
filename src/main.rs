use clap::Parser;
use crate::cli::Args;

mod cli;
fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
