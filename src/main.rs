#![feature(assert_matches)]

mod arguments;
mod cmus;
mod utils;

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


    let sleep = || {
        std::thread::sleep(std::time::Duration::from_millis(args.interval));
    };

    // Initialize the buffer to store the track info, to compare it with the next one.
    let mut previous_track = cmus::Track::default();
    // Initialize the buffer to store the cover path, to compare it with the next one.
    // This is used to speed up the main loop, because we don't need to process the template and search for the cover every time.
    // We only need to do it when the track directory changes.
    let mut previous_cover_path: Option<String> = None;

    loop {
        // Get the track info, and compare it with the previous one.
        let Ok(track) = cmus::ping_cmus(&mut query_command) else {
            if args.link {
                std::process::exit(0)
            } else {
                // If the track info is the same as the previous one, just sleep for a while and try again.
                sleep();
                continue;
            }
        };

/*        // Compare the track info with the previous one, and if they are the same, just sleep for a while and try again.
        if track == previous_track {
            sleep();
            continue;
        }

        // If the track info is different from the previous one, get the changes events.
        let changes = track.get_changes(&previous_track);
*/

        sleep();
    }


}
