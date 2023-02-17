use crate::cmus::player_settings::{AAAMode, Shuffle};
use crate::cmus::{Track, TrackStatus};

#[derive(Debug)]
pub enum CmusEvent {
    StatusChanged(TrackStatus, Track),
    TrackChanged(Track),
    VolumeChanged { left: u8, right: u8 },
    PositionChanged(u32),
    ShuffleChanged(Shuffle),
    RepeatChanged(bool),
    AAAMode(AAAMode),
}
