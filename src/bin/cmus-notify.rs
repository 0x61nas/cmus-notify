use cmus_notify::{
    cmus::{self, query::CmusQueryResponse},
    notification,
    settings::Settings,
};

#[cfg(feature = "debug")]
extern crate pretty_env_logger;
#[cfg(feature = "debug")]
#[macro_use]
extern crate log;

macro_rules! sleep {
    ($time: expr) => {
        #[cfg(feature = "debug")]
        info!("sleeping for {} ms...", $time);
        std::thread::sleep(std::time::Duration::from_millis($time));
    };
}

fn main() {
    #[cfg(feature = "debug")]
    {
        pretty_env_logger::init();
        info!("Starting cmus-notify...");
        info!("Debug mode is enabled. (feature \"debug\")");
        info!("Binary path: {}", file!());
        info!("Parsing the arguments and loading the configs...")
    }
    // Load the configs and parse the arguments, and combine them together.
    let settings = Settings::load_config_and_parse_args();

    #[cfg(feature = "debug")]
    {
        info!("Configs loaded, and arguments parsed.");
        info!("Settings: {:#?}", settings);
    }

    // Build the command, or use the default. (to speed up the main loop, because we don't need to build it every time)
    let mut query_command = cmus::build_query_command(
        settings.remote_bin_path().as_str(),
        &settings.cmus_socket_address,
        &settings.cmus_socket_password,
    );
    #[cfg(feature = "debug")]
    info!("Query command built: {:?}", query_command);

    let interval = settings.interval();
    let link = settings.link;

    let mut notifications_handler = notification::NotificationsHandler::new(settings);

    // Initialize the buffer to store the response from cmus, to compare it with the next one.
    let mut previous_response = CmusQueryResponse::default();

    // Sleep for a 300ms before make the first query, 'cause if the demon linked with `cmus`
    // and the demon is started before `cmus` with the suggested alias, the demon 'll start and exit before `cmus`
    sleep!(300);
    loop {
        // Get the response from cmus.
        let Ok(response) = cmus::ping_cmus(&mut query_command) else {
            if link {
                std::process::exit(0)
            } else {
                // If there is no response and the `link` mode is not active, just sleep and try again
                sleep!(interval);
                continue;
            }
        };

        // Compare the response with the previous one.
        if response != previous_response {
            // Get the events (the changes) from the response.
            if let Ok(events) = previous_response.events(&response) {
                // Update the previous response.
                previous_response = response;

                if !events.is_empty() {
                    match notifications_handler.show_notification(events, &previous_response) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }
        }
        // If the track info is the same as the previous one, just sleep for a while.
        sleep!(interval);
    }
}
