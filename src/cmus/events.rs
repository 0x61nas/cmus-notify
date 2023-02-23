use crate::cmus::player_settings::{AAAMode, PlayerSettings, Shuffle};
use crate::cmus::{Track, TrackStatus};
use crate::notification::Action;
use crate::settings::Settings;
use crate::{process_template_placeholders, settings};

#[derive(PartialEq)]
#[cfg_attr(any(feature = "debug", test), derive(Debug))]
pub enum CmusEvent {
    StatusChanged(Track, PlayerSettings),
    TrackChanged(Track, PlayerSettings),
    VolumeChanged(Track, PlayerSettings),
    PositionChanged(Track, PlayerSettings),
    ShuffleChanged(Track, PlayerSettings),
    RepeatChanged(Track, PlayerSettings),
    AAAMode(Track, PlayerSettings),
}

impl CmusEvent {
    pub fn build_notification(
        &self,
        settings: &Settings,
        notification: &mut notify_rust::Notification,
    ) -> Action {
        use CmusEvent::*;
        match self {
            StatusChanged(_, _) | TrackChanged(_, _) => {
                let (body, body_action) = self.build_notification_body(settings);
                let (summary, summary_action) = self.build_notification_summary(settings);
                notification.body(&body).summary(&summary);
                body_action.max(summary_action)
            }
            VolumeChanged(_, _)
            | PositionChanged(_, _)
            | ShuffleChanged(_, _)
            | RepeatChanged(_, _)
            | AAAMode(_, _)
                if settings.show_player_notifications =>
            {
                let (body, body_action) = self.build_notification_body(settings);
                let (summary, summary_action) = self.build_notification_summary(settings);
                notification.body(&body).summary(&summary);
                body_action.max(summary_action)
            }
            _ => Action::None,
        }
    }

    #[inline]
    fn build_notification_body(&self, settings: &Settings) -> (String, Action) {
        use CmusEvent::*;
        match self {
            StatusChanged(track, player_settings) => (
                process_template_placeholders(
                    settings.status_notification_body(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
            TrackChanged(track, player_settings) => (
                process_template_placeholders(settings.body(), track, player_settings),
                Action::Show,
            ),
            VolumeChanged(track, player_settings) => (
                process_template_placeholders(
                    settings.volume_notification_body(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
            PositionChanged(_track, _player_settings) => {
                (String::new(), Action::None) // TODO: Implement this
            }
            ShuffleChanged(track, player_settings) => (
                process_template_placeholders(
                    settings.shuffle_notification_body(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
            RepeatChanged(track, player_settings) => (
                process_template_placeholders(
                    settings.repeat_notification_body(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
            AAAMode(track, player_settings) => (
                process_template_placeholders(
                    settings.aaa_mode_notification_body(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
        }
    }

    #[inline]
    fn build_notification_summary(&self, settings: &Settings) -> (String, Action) {
        use CmusEvent::*;
        match self {
            StatusChanged(track, player_settings) => (
                process_template_placeholders(
                    settings.status_notification_summary(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
            TrackChanged(track, player_settings) => (
                process_template_placeholders(settings.summary(), track, player_settings),
                Action::Show,
            ),
            VolumeChanged(track, player_settings) => (
                process_template_placeholders(
                    settings.volume_notification_summary(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
            PositionChanged(_track, _player_settings) => {
                (String::new(), Action::None) // TODO: Implement this
            }
            ShuffleChanged(track, player_settings) => (
                process_template_placeholders(
                    settings.shuffle_notification_summary(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
            RepeatChanged(track, player_settings) => (
                process_template_placeholders(
                    settings.repeat_notification_summary(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
            AAAMode(track, player_settings) => (
                process_template_placeholders(
                    settings.aaa_mode_notification_summary(),
                    track,
                    player_settings,
                ),
                Action::Show,
            ),
        }
    }
}
