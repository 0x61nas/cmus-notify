use std::str::FromStr;
use crate::cmus::{CmusError, Track};
use crate::cmus::player_settings::PlayerSettings;

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
}

#[cfg(test)]
mod tests {
    use test_context::{test_context, TestContext};
    use crate::cmus::player_settings::{AAAMode, Shuffle};
    use crate::cmus::TrackStatus;
    use super::*;

    #[test]
    fn test_parse_query_from_str() {
        let row = include_str!("../../tests/samples/row/cmus-remote-output-row.txt");
        let query = CmusQueryResponse::from_str(row);

        assert!(query.is_ok());
        let query = query.unwrap();

        assert_eq!(query.track_row, include_str!("../../tests/samples/row/cmus-remote-output-track-row.txt"));
        assert_eq!(query.player_settings_row, include_str!("../../tests/samples/row/cmus-remote-output-player-row.txt"));
    }

    struct Context {
        query: CmusQueryResponse,
    }

    impl TestContext for Context {
        fn setup() -> Self {
            let row = include_str!("../../tests/samples/row/cmus-remote-output-row.txt");
            let query = CmusQueryResponse::from_str(row).unwrap();

            Self {
                query,
            }
        }
    }

    #[test_context(Context)]
    #[test]
    fn test_actually_parse_the_track_info(ctx: &Context) {
        let track = ctx.query.track();

        assert!(track.is_ok());
        let track = track.unwrap();

        assert_eq!(track.path, "/mnt/Data/Music/FLAC/Taylor Swift/Taylor Swift - Speak Now/12 - Haunted.mp3");
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
