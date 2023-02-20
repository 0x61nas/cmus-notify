use crate::cmus::events::CmusEvent;
use crate::cmus::player_settings::PlayerSettings;
use crate::cmus::{CmusError, Track};
#[cfg(feature = "debug")]
use log::{debug, info};
use std::str::FromStr;

/// This struct is used to store the row status response from cmus.
/// So we don't parse it and take the time then we don't need it.
/// We only parse it when we need it.
#[derive(Debug, PartialEq, Default)]
pub struct CmusQueryResponse {
    track_row: String,
    player_settings_row: String,
}

impl FromStr for CmusQueryResponse {
    type Err = String;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "debug")]
        info!("Parsing cmus response from string: {}", s);

        let sep_index = s.find("set ").ok_or("Corrupted cmus response")?;

        Ok(Self {
            track_row: s[..sep_index].to_string(),
            player_settings_row: s[sep_index..].to_string(),
        })
    }
}

impl CmusQueryResponse {
    /// Actually process and parse the track info, from the cmus response.
    #[inline(always)]
    pub fn track(&self) -> Result<Track, CmusError> {
        Track::from_str(&self.track_row)
    }

    /// Actually process and parse the player settings, from the cmus response.
    #[inline(always)]
    pub fn player_settings(&self) -> Result<PlayerSettings, CmusError> {
        PlayerSettings::from_str(&self.player_settings_row)
    }

    pub fn events(&self, other: &Self) -> Result<Vec<CmusEvent>, CmusError> {
        #[cfg(feature = "debug")]
        info!("Comparing cmus responses: {:?} and {:?}", self, other);

        if self.track_row.is_empty() || self.player_settings_row.is_empty() {
            #[cfg(feature = "debug")]
            info!("Cmus response is empty, returning empty events");
            return Err(CmusError::NoEvents)
        }

        let mut events = Vec::new();

        let track = self.track()?;
        let other_track = other.track()?;

        if track != other_track {
            #[cfg(feature = "debug")]
            debug!("Track changed: {:?} -> {:?}", other_track, track);

            if track.path != other_track.path {
                #[cfg(feature = "debug")]
                debug!("Track changed: {:?} -> {:?}", other_track, track);
                events.push(CmusEvent::TrackChanged(other_track));
                // We don't need to check for other changes, since the track changed.
                return Ok(events);
            } else if track.status != other_track.status {
                #[cfg(feature = "debug")]
                debug!(
                    "Status changed: {:?} -> {:?}",
                    other_track.status, track.status
                );
                events.push(CmusEvent::StatusChanged(track));
            } else if track.position != other_track.position {
                #[cfg(feature = "debug")]
                debug!(
                    "Position changed: {:?} -> {:?}",
                    other_track.position, track.position
                );
                events.push(CmusEvent::PositionChanged(other_track.position));
            }
        }

        let player_settings = self.player_settings()?;
        let other_player_settings = other.player_settings()?;

        if player_settings != other_player_settings {
            #[cfg(feature = "debug")]
            debug!(
                "Player settings changed: {:?} -> {:?}",
                other_player_settings, player_settings
            );

            if player_settings.shuffle != other_player_settings.shuffle {
                #[cfg(feature = "debug")]
                debug!(
                    "Shuffle changed: {:?} -> {:?}",
                    other_player_settings.shuffle, player_settings.shuffle
                );

                events.push(CmusEvent::ShuffleChanged(player_settings.shuffle));
            }

            if player_settings.repeat != other_player_settings.repeat {
                #[cfg(feature = "debug")]
                debug!(
                    "Repeat changed: {:?} -> {:?}",
                    other_player_settings.repeat, player_settings.repeat
                );

                events.push(CmusEvent::RepeatChanged(player_settings.repeat));
            }

            if player_settings.aaa_mode != other_player_settings.aaa_mode {
                #[cfg(feature = "debug")]
                debug!(
                    "AAA mode changed: {:?} -> {:?}",
                    other_player_settings.aaa_mode, player_settings.aaa_mode
                );

                events.push(CmusEvent::AAAMode(player_settings.aaa_mode));
            }

            if player_settings.volume != other_player_settings.volume {
                #[cfg(feature = "debug")]
                debug!(
                    "Volume changed: {:?} -> {:?}",
                    other_player_settings.volume, player_settings.volume
                );

                events.push(CmusEvent::VolumeChanged {
                    left: player_settings.volume.left,
                    right: player_settings.volume.right,
                });
            }
        }

        #[cfg(feature = "debug")]
        info!("Returning events: {:?}", events);

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmus::player_settings::{AAAMode, Shuffle};
    use crate::cmus::TrackStatus;
    use test_context::{test_context, TestContext};

    #[test]
    fn test_parse_query_from_str() {
        let row = include_str!("../../tests/samples/row/cmus-remote-output-row.txt");
        let query = CmusQueryResponse::from_str(row);

        assert!(query.is_ok());
        let query = query.unwrap();

        assert_eq!(
            query.track_row,
            include_str!("../../tests/samples/row/cmus-remote-output-track-row.txt")
        );
        assert_eq!(
            query.player_settings_row,
            include_str!("../../tests/samples/row/cmus-remote-output-player-row.txt")
        );
    }

    struct Context {
        query: CmusQueryResponse,
    }

    impl TestContext for Context {
        fn setup() -> Self {
            let row = include_str!("../../tests/samples/row/cmus-remote-output-row.txt");
            let query = CmusQueryResponse::from_str(row).unwrap();

            Self { query }
        }
    }

    #[test_context(Context)]
    #[test]
    fn test_actually_parse_the_track_info(ctx: &Context) {
        let track = ctx.query.track();

        assert!(track.is_ok());
        let track = track.unwrap();

        assert_eq!(
            track.path,
            "/mnt/Data/Music/FLAC/Taylor Swift/Taylor Swift - Speak Now/12 - Haunted.mp3"
        );
        assert_eq!(track.status, TrackStatus::Playing);
        assert_eq!(track.position, 34);
        assert_eq!(track.duration, 242);
        let metadata = track.metadata;
        assert_eq!(metadata.get("artist"), Some("Taylor Swift"));
        assert_eq!(metadata.get("album"), Some("Speak Now"));
        assert_eq!(metadata.get("title"), Some("Haunted"));
        assert_eq!(metadata.get("date"), Some("2010"));
        assert_eq!(metadata.get("genre"), Some("Pop"));
        assert_eq!(metadata.get("discnumber"), Some("1"));
        assert_eq!(metadata.get("tracknumber"), Some("12"));
        assert_eq!(metadata.get("albumartist"), Some("Taylor Swift"));
        assert_eq!(metadata.get("replaygain_track_gain"), Some("-11.3 dB"));
        assert_eq!(metadata.get("composer"), Some("Taylor Swift"));
        assert_eq!(metadata.get("label"), Some("Big Machine Records, LLC"));
        assert_eq!(metadata.get("publisher"), Some("Big Machine Records, LLC"));
        assert_eq!(metadata.get("bpm"), Some("162"));
        assert_eq!(metadata.get("comment"), None);
    }

    #[test_context(Context)]
    #[test]
    fn test_actually_parse_the_player_settings(ctx: &Context) {
        let player_settings = ctx.query.player_settings();

        assert!(player_settings.is_ok());
        let player_settings = player_settings.unwrap();

        assert_eq!(player_settings.aaa_mode, AAAMode::All);
        assert_eq!(player_settings.repeat, true);
        assert_eq!(player_settings.repeat_current, false);
        assert_eq!(player_settings.shuffle, Shuffle::Off);
        assert_eq!(player_settings.volume.left, 17);
        assert_eq!(player_settings.volume.right, 17);
    }
}
