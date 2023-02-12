use crate::cmus::player_settings::AAAMode;
use crate::cmus::TrackStatus;

pub enum CmusEvent {
    StatusChanged(TrackStatus),
    TrackChanged,
    VolumeChanged { left: u8, right: u8 },
    PositionChanged(u32),
    ShuffleChanged(bool),
    RepeatChanged(bool),
    AAAMode(AAAMode),
}
