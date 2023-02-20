use crate::cmus::events::CmusEvent;
use crate::cmus::{Track, TrackStatus};
use crate::settings::Settings;
use crate::{process_template_placeholders, track_cover, TrackCover};
#[cfg(feature = "debug")]
use log::{debug, info};
use notify_rust::Notification;
use crate::cmus::query::CmusQueryResponse;

#[inline(always)]
pub fn show_notification(
    events: Vec<CmusEvent>,
    settings: &Settings,
    notification: &mut Notification,
    response: &CmusQueryResponse,
) -> Result<(), notify_rust::error::Error> {
    if events.is_empty() {
        #[cfg(feature = "debug")]
        info!("no events to process");
        return Ok(()); // No events to process.
    }

    //FIXME: Should check if the user has enabled the cover feature or use a static cover.
    update_cover(&events[0], response, settings, notification);

    for event in events {
        #[cfg(feature = "debug")]
        info!("event: {:?}", event);

        match event {
            CmusEvent::StatusChanged(track) => {
                #[cfg(feature = "debug")]
                debug!("Status changed: {:?}", track.status);
                build_status_notification(track, settings, notification);
                notification.show()?;
            }
            CmusEvent::TrackChanged(track) => {
                #[cfg(feature = "debug")]
                debug!("Track changed: {:?}", track);
                build_track_notification(track, settings, notification)?
            }
            /*
                        CmusEvent::VolumeChanged { left, right } if settings.show_player_notifications => {
                            build_volume_notification(left, right, settings, notification)?
                        }
                        CmusEvent::PositionChanged(position) => todo!(),
                        CmusEvent::ShuffleChanged(shuffle) if settings.show_player_notifications => {
                            build_shuffle_notification(shuffle, settings, notification)?
                        }
                        CmusEvent::RepeatChanged(repeat) if settings.show_player_notifications => {
                            build_repeat_notification(repeat, settings, notification)?
                        }
                        CmusEvent::AAAMode(aaa_mode) if settings.show_player_notifications => {
                            build_aaa_mode_notification(aaa_mode, settings, notification)?
                        }
            */
            _ => {}
        }
    }
    Ok(())
}

#[inline(always)]
fn build_status_notification(
    track: Track,
    settings: &Settings,
    notification: &mut Notification,
) {
    // Set the summary and body of the notification.
    notification
        .summary(
            process_template_placeholders(&settings.status_notification_summary, &track).as_str(),
        )
        .body(process_template_placeholders(&settings.status_notification_body, &track).as_str())
        .timeout(settings.status_notification_timeout as i32 * 1000);
}

#[inline(always)]
fn build_track_notification(
    track: Track,
    settings: &Settings,
    notification: &mut Notification,
) -> Result<(), notify_rust::error::Error> {
    // Set the summary and body of the notification.
    notification
        .summary(process_template_placeholders(&settings.summary, &track).as_str())
        .body(process_template_placeholders(&settings.body, &track).as_str());

    let n = notification.show()?;

    Ok(())
}

macro_rules! setup_notification {
    ($notification: expr, $settings: expr) => {
        $notification
        .appname("cmus-notify")
        .timeout($settings.timeout as i32 * 1000)
        .hint(notify_rust::Hint::Category("music".to_string()))
        .hint(notify_rust::Hint::DesktopEntry("cmus-notify.desktop".to_string()))
        .hint(notify_rust::Hint::Resident(true));
    };
}

#[inline(always)]
fn update_cover(first_event: &CmusEvent, response: &CmusQueryResponse, settings: &Settings, notification: &mut Notification) {
    let mut cover = TrackCover::None;
    // If the track is changed, we need to update the cover.
    match first_event {
        CmusEvent::TrackChanged(track) => {
            cover = track_cover(
                &track.path,
                settings.depth,
                settings.force_use_external_cover,
                settings.no_use_external_cover,
            );
        }
        _ => {
            if cover == TrackCover::None {
                // If the cover is not found, we need to update it.
                if let Ok(track) = &response.track() {
                    cover = track_cover(
                        &track.path,
                        settings.depth,
                        settings.force_use_external_cover,
                        settings.no_use_external_cover,
                    );
                }
            }
        }
    };
    // Set the notification cover.
    if cover != TrackCover::None {
        *notification = Notification::new();
        // Reset the notification.
        setup_notification!(&mut *notification, &settings);
        cover.set_notification_image(&mut *notification);
    }
}
