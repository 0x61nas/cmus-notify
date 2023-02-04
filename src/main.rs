#![feature(assert_matches)]

mod arguments;
mod cmus;

use clap::Parser;

fn main() {
    let _args = arguments::Arguments::parse();
    println!("{_args:#?}");
}
