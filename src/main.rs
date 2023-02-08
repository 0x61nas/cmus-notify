#![feature(assert_matches)]

mod arguments;
mod cmus;

use clap::Parser;

fn main() {
    let args = arguments::Arguments::parse();

    // Build the command, or use the default. (to speed up the main loop, because we don't need to build it every time)
    let mut query_command = cmus::build_query_command(
        &args
            .cmus_remote_bin_path
            .unwrap_or("cmus-remote".to_string())
            .as_str(),
        &args.cmus_socket_address,
        &args.cmus_socket_password,
    );
}
