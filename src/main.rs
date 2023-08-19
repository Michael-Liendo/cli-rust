#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: String,
}

fn main() {
    let args = Cli::parse();
}
