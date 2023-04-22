#[cfg(feature = "debug")]
use log::{info};
use notify_rust::Notification;

use crate::{track_cover, TrackCover};
use crate::cmus::{TemplateProcessor, Track};
use crate::cmus::events::CmusEvent;

use crate::cmus::query::CmusQueryResponse;
use crate::settings::Settings;

pub enum Action {
    Show {
        notification_body: String,
        notification_summary: String,
        save: bool,
    },
    None,
}

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
            handlers: Vec::with_capacity(2),
            settings,
        }
    }

    #[inline]
    pub fn show_notification(
        &mut self,
        events: Vec<CmusEvent>,
        response: &CmusQueryResponse,
    ) -> Result<(), notify_rust::error::Error> {
        for event in events {
            self.setup_notification_timeout(&event);
            #[cfg(feature = "debug")]
            info!("event: {:?}", event);

            match event.build_notification(&self.settings) {
                Action::Show { notification_body, notification_summary, save } => {
                    // Setup the notification cover
                    if self.settings.show_track_cover {
                        self.update_cover(&event, response);
                    } else if self.settings.notification_static_cover.is_some() && !self.cover_set {
                        self.setup_the_notification();
                        self.notification
                            .image_path(self.settings.notification_static_cover.as_ref().unwrap());
                        self.cover_set = true;
                    }

                    self.notification.summary(&notification_summary).body(&notification_body);

                    // Show the notification
                    let handle = self.notification.show()?;
                    if save {
                        self.handlers.push(handle);
                    }
                }
                Action::None => {}
            };
        }

        Ok(())
    }

    #[inline(always)]
    fn update_cover(&mut self, event: &CmusEvent, response: &CmusQueryResponse) {
        // If the track is changed, we need to update the cover.
        match event {
            CmusEvent::TrackChanged(track, _) => {
                // Reset the notification
                self.setup_the_notification();
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

    #[inline]
    fn set_cover(&mut self, track: &Track) {
        let path = match &self.settings.cover_path_template {
            Some(template) => track.process(template.clone()),
            None => track.path.clone(),
        };
        // Get the track cover and set it to notification
        let track_cover = track_cover(
            path,
            track.get_name(),
            self.settings.depth(),
            self.settings.force_use_external_cover,
            self.settings.no_use_external_cover,
        );

        if track_cover != TrackCover::None {
            track_cover.set_notification_image(&mut self.notification);
        } else if self.settings.notification_static_cover.is_some() {
            self.notification
                .image_path(self.settings.notification_static_cover.as_ref().unwrap());
        }

        // Flip the change flag
        self.cover_set = true;
    }

    #[inline(always)]
    fn setup_the_notification(&mut self) {
        self.notification = Notification::new();
        self.notification
            .appname(self.settings.app_name().as_str())
            .hint(notify_rust::Hint::Category("music".to_string()))
            .hint(notify_rust::Hint::DesktopEntry("cmus.desktop".to_string()))
            .hint(notify_rust::Hint::Resident(true));
    }

    #[inline(always)]
    fn setup_notification_timeout(&mut self, event: &CmusEvent) {
        use CmusEvent::*;
        self.notification.timeout(
            match event {
                TrackChanged(_, _) => self.settings.timeout(),
                StatusChanged(_, _) => self.settings.status_notification_timeout(),
                AAAModeChanged(_, _) => self.settings.aaa_mode_notification_timeout(),
                VolumeChanged(_, _) => self.settings.volume_notification_timeout(),
                RepeatChanged(_, _) => self.settings.repeat_notification_timeout(),
                ShuffleChanged(_, _) => self.settings.shuffle_notification_timeout(),
                _ => self.settings.timeout(),
            } as i32
                * 1000,
        );
    }
}
