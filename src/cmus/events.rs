use crate::cmus::player_settings::{AAAMode, PlayerSettings, Shuffle};
use crate::cmus::{Track, TrackStatus};

#[derive(Debug, PartialEq)]
pub enum CmusEvent {
    StatusChanged(Track, PlayerSettings),
    TrackChanged(Track, PlayerSettings),
    VolumeChanged(Track, PlayerSettings),
    PositionChanged(Track, PlayerSettings),
    ShuffleChanged(Track, PlayerSettings),
    RepeatChanged(Track, PlayerSettings),
    AAAMode(Track, PlayerSettings),
}
