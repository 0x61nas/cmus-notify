use std::num::ParseIntError;
use std::str::FromStr;
use crate::cmus::CmusError;

#[derive(Debug, PartialEq)]
pub struct PlayerSettings {
    pub repeat: bool,
    pub shuffle: bool,
    pub aa_mode: AAAMode,
    pub volume: Volume,
}

#[derive(Debug, PartialEq, Default)]
pub struct Volume {
    pub left: u8,
    pub right: u8,
}

#[derive(Debug, PartialEq, Default)]
pub enum AAAMode {
    All,
    Album,
    Artist,
    #[default]
    None,
}

impl FromStr for AAAMode {
    type Err = CmusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "album" => Ok(Self::Album),
            "artist" => Ok(Self::Artist),
            "none" => Ok(Self::None),
            _ => Err(CmusError::UnknownAAAMode(s.to_string()))
        }
    }
}

impl FromStr for PlayerSettings {
    type Err = CmusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut repeat = false;
        let mut shuffle = false;
        let mut aa_mode = AAAMode::default();
        let mut volume = Volume::default();

        for line in s.lines() {
            if line.starts_with("set ") {
                let line = &line[4..];
                let (key, value) = line.split_once(" ").ok_or("Corrupted cmus response")?;

                match key {
                    "repeat" => repeat = value == "true",
                    "shuffle" => shuffle = value == "true",
                    "aa_mode" => aa_mode = AAAMode::from_str(value)?,
                    "vol_left" => volume.left = value.parse().map_err(|e: ParseIntError| CmusError::UnknownError(e.to_string()))?,
                    "vol_right" => volume.right = value.parse().map_err(|e: ParseIntError| CmusError::UnknownError(e.to_string()))?,
                    _ => {}
                }
            }
        }

        Ok(Self {
            repeat,
            shuffle,
            aa_mode,
            volume,
        })
    }
}
