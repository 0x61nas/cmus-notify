use crate::{CompleteStr, process_template_placeholders};
use crate::cmus::Track;
use crate::cmus::player_settings::PlayerSettings;
use crate::notification::Action;
use crate::settings::Settings;

#[derive(PartialEq)]
#[cfg_attr(any(feature = "debug", test), derive(Debug))]
pub enum CmusEvent {
    StatusChanged(Track, PlayerSettings),
    TrackChanged(Track, PlayerSettings),
    VolumeChanged(Track, PlayerSettings),
    PositionChanged(Track, PlayerSettings),
    ShuffleChanged(Track, PlayerSettings),
    RepeatChanged(Track, PlayerSettings),
    AAAModeChanged(Track, PlayerSettings),
}

impl CmusEvent {
    pub fn build_notification(
        &self,
        settings: &Settings,
    ) -> Action {
        use CmusEvent::*;
        let (body_template, summary_template, timeout, track, player_settings) = match self {
            StatusChanged(track, player_settings) =>
                (settings.status_notification_body(), settings.status_notification_summary(), settings.status_notification_timeout(), track, player_settings),
            TrackChanged(track, player_settings) =>
                (settings.body(), settings.summary(), settings.timeout(), track, player_settings),
            VolumeChanged(track, player_settings) if settings.show_player_notifications =>
                (settings.volume_notification_body(), settings.volume_notification_summary(), settings.volume_notification_timeout(), track, player_settings),
            ShuffleChanged(track, player_settings) if settings.show_player_notifications =>
                (settings.shuffle_notification_body(), settings.shuffle_notification_summary(), settings.shuffle_notification_timeout(), track, player_settings),
            RepeatChanged(track, player_settings) if settings.show_player_notifications =>
                (settings.repeat_notification_body(), settings.repeat_notification_summary(), settings.repeat_notification_timeout(), track, player_settings),
            AAAModeChanged(track, player_settings) if settings.show_player_notifications =>
                (settings.aaa_mode_notification_body(), settings.aaa_mode_notification_summary(), settings.aaa_mode_notification_timeout(), track, player_settings),
            _ => { return Action::None },
        };

        let persistent = is_mutable(&body_template) || is_mutable(&summary_template);

        Action::Show {
            body: CompleteStr {
                template: body_template.clone(),
                str: process_template_placeholders(
                    body_template,
                    track,
                    player_settings,
                ),
            },
            summary: CompleteStr {
                template: summary_template.clone(),
                str: process_template_placeholders(
                    summary_template,
                    track,
                    player_settings,
                ),
            },
            timeout: if persistent { 0 } else { timeout * 1000 },
            save: persistent,
        }
    }
}

fn is_mutable(template: &str) -> bool {
        let mut key = String::new(); // Just a buffer to build the key.

        for c in template.chars() {
            if c == '{' {
                key = String::new();
            } else if c == '}' {
                match key.as_str() {
                    "lyrics" | "progress" | "progress_bar" => return true,
                    _ => {}
                }
            } else {
                key.push(c);
            }
        }

    false
}
