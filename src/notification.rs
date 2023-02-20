use crate::cmus::events::CmusEvent;
use crate::cmus::{Track, TrackStatus};
use crate::settings::Settings;
use crate::{process_template_placeholders, track_cover, TrackCover};
#[cfg(feature = "debug")]
use log::{debug, info};
use notify_rust::Notification;
use crate::cmus::query::CmusQueryResponse;

pub struct NotificationsHandler {
    cover_set: bool,
    notification: Notification,
    handlers: Vec<notify_rust::NotificationHandle>,
    settings: Settings,
}

impl NotificationsHandler {
    pub fn new(settings: Settings) -> Self {
        Self {
            cover_set: false,
            notification: Notification::new(),
            handlers: Vec::new(),
            settings,
        }
    }

    pub fn show_notification(
        &mut self,
        events: Vec<CmusEvent>,
        response: &CmusQueryResponse,
    ) -> Result<(), notify_rust::error::Error> {
        if events.is_empty() {
            #[cfg(feature = "debug")]
            info!("no events to process");
            return Ok(()); // No events to process.
        }

        //FIXME: Should check if the user has enabled the cover feature or use a static cover.
        self.update_cover(&events[0], response);

        for event in events {
            #[cfg(feature = "debug")]
            info!("event: {:?}", event);

            match event {
                CmusEvent::StatusChanged(track) => {
                    #[cfg(feature = "debug")]
                    debug!("Status changed: {:?}", track.status);
                    self.build_status_notification(track);
                    self.notification.show()?;
                }
                CmusEvent::TrackChanged(track) => {
                    #[cfg(feature = "debug")]
                    debug!("Track changed: {:?}", track);
                    self.build_track_notification(track)?
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
    fn build_status_notification(&mut self, track: Track) {
        // Set the summary and body of the notification.
        self.notification
            .summary(
                process_template_placeholders(&self.settings.status_notification_summary, &track).as_str(),
            )
            .body(process_template_placeholders(&self.settings.status_notification_body, &track).as_str())
            .timeout(self.settings.status_notification_timeout as i32 * 1000);
    }

    #[inline(always)]
    fn build_track_notification(&mut self, track: Track) -> Result<(), notify_rust::error::Error> {
        // Set the summary and body of the notification.
        self.notification
            .summary(process_template_placeholders(&self.settings.summary, &track).as_str())
            .body(process_template_placeholders(&self.settings.body, &track).as_str());

        let n = self.notification.show()?;

        Ok(())
    }

    #[inline(always)]
    fn update_cover(&mut self, first_event: &CmusEvent, response: &CmusQueryResponse) {
        // If the track is changed, we need to update the cover.
        match first_event {
            CmusEvent::TrackChanged(track) => {
                self.set_cover(track);
            }
            _ => {
                if !self.cover_set {
                    // If the cover is not found, we need to update it.
                    if let Ok(track) = response.track() {
                        self.set_cover(&track);
                    }
                }
            }
        };

    }

    fn set_cover(&mut self, track: &Track) {
        // Reset the notification
        self.notification = Notification::new();
        self.notification.appname("cmus-notify")
            .timeout(self.settings.timeout as i32 * 1000)
            .hint(notify_rust::Hint::Category("music".to_string()))
            .hint(notify_rust::Hint::DesktopEntry("cmus.desktop".to_string()))
            .hint(notify_rust::Hint::Resident(true));

        // Get the track cover and set it to notification
        track_cover(
            &track.path,
            self.settings.depth,
            self.settings.force_use_external_cover,
            self.settings.no_use_external_cover,
        ).set_notification_image(&mut self.notification);
        // Flip the change flag
        self.cover_set = true;
    }
}

