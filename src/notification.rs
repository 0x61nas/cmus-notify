use crate::cmus::events::CmusEvent;
use crate::cmus::player_settings::PlayerSettings;
use crate::cmus::query::CmusQueryResponse;
use crate::cmus::{TemplateProcessor, Track, TrackStatus};
use crate::settings::Settings;
use crate::{process_template_placeholders, settings, track_cover, TrackCover};
#[cfg(feature = "debug")]
use log::{debug, info};
use notify_rust::Notification;

pub enum Action {
    Show,
    Update,
    None,
}

impl Action {
    pub fn max(self, other: Self) -> Self {
        match (self, other) {
            (Action::Show, Action::Show) => Action::Show,
            (Action::Show, Action::Update) | (Action::Update, Action::Show | Action::Update) => {
                Action::Update
            }
            (Action::None, _) => Action::None,
            (_, Action::None) => Action::None,
        }
    }
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
            handlers: Vec::new(),
            settings,
        }
    }

    #[inline]
    pub fn show_notification(
        &mut self,
        events: Vec<CmusEvent>,
        response: &CmusQueryResponse,
    ) -> Result<(), notify_rust::error::Error> {
        // Setup the notification cover
        if self.settings.show_track_cover {
            self.update_cover(&events[0], response);
        } else if self.settings.notification_static_cover.is_some() && !self.cover_set {
            self.setup_the_notification();
            self.notification
                .image_path(self.settings.notification_static_cover.as_ref().unwrap());
            self.cover_set = true;
        }

        for event in events {
            self.setup_notification_timeout(&event);
            #[cfg(feature = "debug")]
            info!("event: {:?}", event);

            let action = event.build_notification(&mut self.settings, &mut self.notification);

            match action {
                Action::Show => {
                    let _ = self.notification.show()?;
                }
                Action::Update => todo!("Update notification"),
                Action::None => {}
            };
        }

        Ok(())
    }

    #[inline(always)]
    fn update_cover(&mut self, first_event: &CmusEvent, response: &CmusQueryResponse) {
        // If the track is changed, we need to update the cover.
        match first_event {
            CmusEvent::TrackChanged(track, _) => {
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
        // Reset the notification
        self.setup_the_notification();
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
                AAAMode(_, _) => self.settings.aaa_mode_notification_timeout(),
                VolumeChanged(_, _) => self.settings.volume_notification_timeout(),
                RepeatChanged(_, _) => self.settings.repeat_notification_timeout(),
                ShuffleChanged(_, _) => self.settings.shuffle_notification_timeout(),
                _ => self.settings.timeout(),
            } as i32
                * 1000,
        );
    }
}
