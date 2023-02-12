use clap::Parser;

use cmus_notify::{
    arguments,
    cmus::{self, query::CmusQueryResponse, CmusError},
};
use cmus_notify::arguments::Arguments;

macro_rules! sleep {
    ($time: expr) => {
        std::thread::sleep(std::time::Duration::from_millis($time));
    };
}

fn main() {
    // Load the configs
    let settings = Arguments::load_config_and_parse_args();

    // Build the command, or use the default. (to speed up the main loop, because we don't need to build it every time)
    let mut query_command = cmus::build_query_command(
        &args
            .cmus_remote_bin_path
            .unwrap_or("cmus-remote".to_string())
            .as_str(),
        &args.cmus_socket_address,
        &args.cmus_socket_password,
    );

    // Initialize the buffer to store the response from cmus, to compare it with the next one.
    let mut previous_response = CmusQueryResponse::default();
    // Initialize the buffer to store the cover path, to compare it with the next one.
    // This is used to speed up the main loop, because we don't need to process the template and search for the cover every time.
    // We only need to do it when the track directory changes.
    let mut previous_cover_path: Option<String> = None;

    loop {
        // Get the track info, and compare it with the previous one.
        let Ok(response) = cmus::ping_cmus(&mut query_command) else {
            if args.link {
                std::process::exit(0)
            } else {
                // If the track info is the same as the previous one, just sleep for a while and try again.
                sleep!(args.interval);
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

        sleep!(args.interval);
    }
}
