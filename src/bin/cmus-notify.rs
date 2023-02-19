use cmus_notify::{
    cmus::{self, events::CmusEvent, query::CmusQueryResponse},
    notification,
    settings::Settings,
    track_cover, TrackCover,
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
    let remote_bin_path = settings
        .cmus_remote_bin_path
        .clone()
        .unwrap_or("cmus-remote".to_string());
    let mut query_command = cmus::build_query_command(
        remote_bin_path.as_str(),
        &settings.cmus_socket_address,
        &settings.cmus_socket_password,
    );
    #[cfg(feature = "debug")]
    info!("Query command built: {:?}", query_command);

    let mut notification = notify_rust::Notification::new();

    // Initialize the buffer to store the response from cmus, to compare it with the next one.
    let mut previous_response = CmusQueryResponse::default();
    // Initialize the buffer to store the cover path, to compare it with the next one.
    // This is used to speed up the main loop, because we don't need to process the template and search for the cover every time.
    // We only need to do it when the track directory changes.
    let mut cover = TrackCover::None;

    loop {
        // Get the response from cmus.
        let Ok(response) = cmus::ping_cmus(&mut query_command) else {
            if settings.link {
                std::process::exit(0)
            } else {
                // If the track info is the same as the previous one, just sleep for a while and try again.
                sleep!(settings.interval);
                continue;
            }
        };

        // Compare the response with the previous one.
        if response != previous_response {
            // Get the events (the changes) from the response.
            if let Ok(events) = previous_response.events(&response) {
                // Update the previous response.
                previous_response = response;

                //FIXME: Should check if the user has enabled the cover feature or use a static cover.
                if events.len() == 1 {
                    // If the track is changed, we need to update the cover.
                    let mut cover_changed = false;
                    match &events[0] {
                        CmusEvent::TrackChanged(track) => {
                            cover = track_cover(
                                &track.path,
                                settings.depth,
                                settings.force_use_external_cover,
                                settings.no_use_external_cover,
                            );
                            cover_changed = true;
                        }
                        _ => {
                            if cover == TrackCover::None {
                                // If the cover is not found, we need to update it.
                                if let Ok(track) = &previous_response.track() {
                                    cover = track_cover(
                                        &track.path,
                                        settings.depth,
                                        settings.force_use_external_cover,
                                        settings.no_use_external_cover,
                                    );
                                    cover_changed = true;
                                }
                            }
                        }
                    };
                    // Set the notification cover.
                    if cover_changed {
                        notification = notify_rust::Notification::new(); // Reset the notification.
                        cover.set_notification_image(&mut notification);
                    }
                }

                match notification::show_notification(events, &settings, &mut notification) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
        sleep!(settings.interval);
    }
}
