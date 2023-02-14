use crate::cmus::events::CmusEvent;
use crate::settings::Settings;
use crate::TrackCover;
#[cfg(feature = "debug")]
use log::info;

#[inline(always)]
pub fn show_notification(
    events: Vec<CmusEvent>,
    settings: &Settings,
    previous_cover: &mut TrackCover,
) -> Result<(), notify_rust::error::Error> {
    if events.is_empty() {
        #[cfg(feature = "debug")]
        info!("no events to process");
        return Ok(()); // No events to process.
    }

    let mut notification = notify_rust::Notification::new();

    for event in events {
        #[cfg(feature = "debug")]
        info!("event: {:?}", event);
        match event {
            CmusEvent::StatusChanged(status) => {
                println!("{:?}", status);
            }
            CmusEvent::TrackChanged(track) => {
                println!("track change {:?}", track);
            }
            CmusEvent::VolumeChanged { left, right } if settings.show_player_notifications => {
                println!("left: {}, right: {}", left, right);
            }
            CmusEvent::PositionChanged(position) => {
                println!("position: {}", position);
            }
            CmusEvent::ShuffleChanged(shuffle) if settings.show_player_notifications => {
                println!("shuffle: {:?}", shuffle);
            }
            CmusEvent::RepeatChanged(repeat) if settings.show_player_notifications => {
                println!("repeat: {}", repeat);
            }
            CmusEvent::AAAMode(aaa_mode) if settings.show_player_notifications => {
                println!("aaa_mode: {:?}", aaa_mode);
            }
            _ => {}
        }
        notification.show()?;
    }
    Ok(())
}
