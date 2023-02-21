use crate::cmus::{CmusError, TemplateProcessor};
#[cfg(feature = "debug")]
use log::{debug, info};
use parse_display::Display;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerSettings {
    pub repeat: bool,
    pub repeat_current: bool,
    pub shuffle: Shuffle,
    pub aaa_mode: AAAMode,
    pub volume: Volume,
}

#[derive(Display, Debug, PartialEq, Default, Clone)]
pub enum Shuffle {
    #[default]
    Off,
    Tracks,
    Albums,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Volume {
    pub left: u8,
    pub right: u8,
}

#[derive(Display, Debug, PartialEq, Default, Clone)]
pub enum AAAMode {
    #[default]
    All,
    Album,
    Artist,
}

impl TemplateProcessor for PlayerSettings {
    fn process(&self, template: &String) -> String {
        #[cfg(feature = "debug")]
        {
            info!("Processing template: {}", template);
            debug!("Processing template with player settings: {:?}", self);
        }
        let mut processed = template.clone();

        Self::get_keys(template).iter().for_each(|key| {
            let value = match key.as_str() {
                "repeat" => self.repeat.to_string(),
                "repeat_current" => self.repeat_current.to_string(),
                "shuffle" => self.shuffle.to_string(),
                "aaa_mode" => self.aaa_mode.to_string(),
                "volume_left" => self.volume.left.to_string(),
                "volume_right" => self.volume.right.to_string(),
                "volume" => {
                    if self.volume.left == self.volume.right {
                        self.volume.left.to_string()
                    } else {
                        format!("{}:{}", self.volume.left, self.volume.right)
                    }
                }
                _ => "".to_string(),
            };
            processed = processed.replace(&format!("{{{key}}}"), &value);
        });

        #[cfg(feature = "debug")]
        info!("Processed template: {}", processed);

        processed
    }
}

impl FromStr for AAAMode {
    type Err = CmusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "album" => Ok(Self::Album),
            "artist" => Ok(Self::Artist),
            _ => Err(CmusError::UnknownAAAMode(s.to_string())),
        }
    }
}

impl FromStr for Shuffle {
    type Err = CmusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "tracks" => Ok(Self::Tracks),
            "albums" => Ok(Self::Albums),
            _ => Err(CmusError::UnknownShuffleMode(s.to_string())),
        }
    }
}

impl FromStr for PlayerSettings {
    type Err = CmusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "debug")]
        info!("Parsing cmus response from string: {}", s);
        let mut repeat = false;
        let mut repeat_current = false;
        let mut shuffle = Shuffle::default();
        let mut aaa_mode = AAAMode::default();
        let mut volume = Volume::default();

        for line in s.lines() {
            #[cfg(feature = "debug")]
            debug!("Parsing line: {}", line);
            if line.starts_with("set ") {
                let line = &line[4..];
                let (key, value) = line.split_once(" ").ok_or(CmusError::UnknownError(
                    "Corrupted cmus response".to_string(),
                ))?;

                match key {
                    "repeat" => repeat = value == "true",
                    "repeat_current" => repeat_current = value == "true",
                    "shuffle" => shuffle = Shuffle::from_str(value)?,
                    "aaa_mode" => aaa_mode = AAAMode::from_str(value)?,
                    "vol_left" => {
                        volume.left = value
                            .parse()
                            .map_err(|e: ParseIntError| CmusError::UnknownError(e.to_string()))?
                    }
                    "vol_right" => {
                        volume.right = value
                            .parse()
                            .map_err(|e: ParseIntError| CmusError::UnknownError(e.to_string()))?
                    }
                    _ => {}
                }
            }
        }

        Ok(Self {
            repeat,
            repeat_current,
            shuffle,
            aaa_mode,
            volume,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_aaamode_from_str() {
        let all = AAAMode::from_str("all");
        let album = AAAMode::from_str("album");
        let artist = AAAMode::from_str("artist");
        let unknown = AAAMode::from_str("unknown");

        assert_eq!(all, Ok(AAAMode::All));
        assert_eq!(album, Ok(AAAMode::Album));
        assert_eq!(artist, Ok(AAAMode::Artist));
        assert_eq!(
            unknown,
            Err(CmusError::UnknownAAAMode("unknown".to_string()))
        );
    }

    #[test]
    fn test_parse_shuffle_mode_from_str() {
        let off = Shuffle::from_str("off");
        let tracks = Shuffle::from_str("tracks");
        let albums = Shuffle::from_str("albums");
        let unknown = Shuffle::from_str("unknown");

        assert_eq!(off, Ok(Shuffle::Off));
        assert_eq!(tracks, Ok(Shuffle::Tracks));
        assert_eq!(albums, Ok(Shuffle::Albums));
        assert_eq!(
            unknown,
            Err(CmusError::UnknownShuffleMode("unknown".to_string()))
        );
    }

    #[test]
    fn test_parse_player_settings_from_str() {
        let setting_sample = include_str!(
            "../../tests/samples/player_settings_mode-artist_vol-46_repeat-false_repeat_current-false_shuffle-tracks.txt");

        let settings = PlayerSettings::from_str(setting_sample);

        assert_eq!(
            settings,
            Ok(PlayerSettings {
                repeat: false,
                repeat_current: false,
                shuffle: Shuffle::Tracks,
                aaa_mode: AAAMode::Artist,
                volume: Volume {
                    left: 46,
                    right: 46,
                },
            })
        );
    }
}
