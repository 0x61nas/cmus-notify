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
    AAAModeChanged(Track, PlayerSettings),
}

impl CmusEvent {
    pub fn build_notification(
        &self,
        settings: &Settings,
        _notification: &mut notify_rust::Notification,
    ) -> Action {
        macro_rules! build_the_notification {
            ($body_template: expr, $summary_template: expr, $track: expr, $player_settings: expr) => {
                _notification.body(&process_template_placeholders(
                    $body_template,
                    $track,
                    $player_settings,
                ))
                    .summary(&process_template_placeholders(
                    $summary_template,
                    $track,
                    $player_settings,
                ));
            };
        }

        use CmusEvent::*;
        match self {
            StatusChanged(track, player_settings) => {
                build_the_notification!(settings.status_notification_body(), settings.status_notification_summary(), track, player_settings);

                Action::Show
            }
            TrackChanged(track, player_settings) => {
                build_the_notification!(settings.body(), settings.summary(), track, player_settings);

                Action::Show
            }
            VolumeChanged(track, player_settings) if settings.show_player_notifications => {
                build_the_notification!(settings.volume_notification_body(), settings.volume_notification_summary(), track, player_settings);

                Action::Show
            }
            PositionChanged(_track, _player_settings) if settings.show_player_notifications => {
                //build_the_notification!(settings.volume_notification_body(), settings.volume_notification_summary(), track, player_settings);
                //TODO: Implement this
                Action::None
            }
            ShuffleChanged(track, player_settings) if settings.show_player_notifications => {
                build_the_notification!(settings.shuffle_notification_body(), settings.shuffle_notification_summary(), track, player_settings);

                Action::Show
            }
            RepeatChanged(track, player_settings) if settings.show_player_notifications => {
                build_the_notification!(settings.repeat_notification_body(), settings.repeat_notification_summary(), track, player_settings);

                Action::Show
            }
            AAAModeChanged(track, player_settings) if settings.show_player_notifications => {
                build_the_notification!(settings.aaa_mode_notification_body(), settings.aaa_mode_notification_summary(), track, player_settings);

                Action::Show
            }
            _ => Action::None,
        }
    }
}
