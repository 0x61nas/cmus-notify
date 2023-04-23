#[cfg(feature = "debug")]
use log::info;
use notify_rust::Notification;

use crate::{CompleteStr, track_cover, TrackCover};
use crate::cmus::{TemplateProcessor, Track};
use crate::cmus::events::CmusEvent;
use crate::cmus::player_settings::PlayerSettings;
use crate::cmus::query::CmusQueryResponse;
use crate::settings::Settings;

pub enum Action {
    Show {
        body: CompleteStr,
        summary:CompleteStr,
        timeout: i32,
        save: bool,
    },
    None,
}

pub struct NotificationsHandler {
    cover_set: bool,
    notification: Notification,
    notifications: Vec<CmusNotification>,
    settings: Settings,
}

struct CmusNotification {
    body_template: String,
    summary_template: String,
    visible: bool,
    handle: notify_rust::NotificationHandle
}

impl CmusNotification {
    #[inline(always)]
    fn update(&mut self, track: &Track, player_settings: &PlayerSettings) {
        use crate::process_template_placeholders;
        self.handle.summary(&process_template_placeholders(self.summary_template.clone(), track, player_settings))
            .body(&process_template_placeholders(self.body_template.clone(), track, player_settings));
        self.handle.update();
    }
}

impl NotificationsHandler {
    pub fn new(settings: Settings) -> Self {
        Self {
            cover_set: false,
            notification: Notification::new(),
            notifications: Vec::with_capacity(2),
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
            #[cfg(feature = "debug")]
            info!("event: {:?}", event);

            if let CmusEvent::PositionChanged(track, player_settings) = &event {
                for notification in &mut self.notifications {
                    if notification.visible {
                        notification.update(track, player_settings);
                    }
                }
                continue;
            } else if let CmusEvent::TrackChanged(_, _) = &event {
                for notification in &mut self.notifications {
                    notification.handle.timeout = 2.into(); // Hide the notification after 2 millisecond
                    notification.handle.update();
                }
                // Clean the notifications vec
                self.notifications.clear();
            }

            match event.build_notification(&self.settings) {
                Action::Show { body, summary, timeout, save } => {
                    // Setup the notification cover
                    if self.settings.show_track_cover {
                        self.update_cover(&event, response);
                    } else if self.settings.notification_static_cover.is_some() && !self.cover_set {
                        self.setup_the_notification();
                        self.notification
                            .image_path(self.settings.notification_static_cover.as_ref().unwrap());
                        self.cover_set = true;
                    }

                    self.notification.timeout(timeout).summary(&summary.str).body(&body.str);

                    // Show the notification
                    let mut handle = self.notification.show()?;
                    if save {
                        // Add the close handler
                        /*handle.on_close(|reason| {

                        });*/

                        self.notifications.push(
                            CmusNotification {
                                body_template: body.template,
                                summary_template: summary.template,
                                visible: true,
                                handle
                            }
                        )
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
}
