mod arguments;

use clap::Parser;

fn main() {
    let _args = arguments::Arguments::parse();
    println!("{_args:#?}");
}
