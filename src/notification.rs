use crate::cmus::events::CmusEvent;
use crate::cmus::player_settings::PlayerSettings;
use crate::cmus::query::CmusQueryResponse;
use crate::cmus::{Track, TrackStatus};
use crate::settings::Settings;
use crate::{process_template_placeholders, track_cover, TrackCover};
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
        //FIXME: Should check if the user has enabled the cover feature or use a static cover.
        self.update_cover(&events[0], response);

        let mut action = Action::None;

        for event in events {
            #[cfg(feature = "debug")]
            info!("event: {:?}", event);

            action = event.build_notification(&mut self.settings, &mut self.notification);

            match action {
                Action::Show => {
                    let _ = self.notification.show()?;
                }
                Action::None => {}
                _ => todo!(),
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
        self.notification = Notification::new();
        self.notification
            .appname("cmus-notify")
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
        )
        .set_notification_image(&mut self.notification);
        // Flip the change flag
        self.cover_set = true;
    }
}
