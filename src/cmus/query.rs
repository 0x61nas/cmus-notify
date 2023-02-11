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
