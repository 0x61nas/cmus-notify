use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use typed_builder::TypedBuilder;

#[derive(Debug, PartialEq)]
pub struct TrackMetadata {
    tags: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum TrackStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, TypedBuilder, PartialEq)]
pub struct Track {
    pub status: TrackStatus,
    pub path: String,
    pub metadata: TrackMetadata,
    pub duration: u32,
    pub position: u32,
}

impl FromStr for TrackStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "playing" => Ok(TrackStatus::Playing),
            "paused" => Ok(TrackStatus::Paused),
            "stopped" => Ok(TrackStatus::Stopped),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

impl FromStr for Track {
    type Err = String;

    /// Creates a `Track` from the output of `cmus-remote -Q`.
    ///
    /// Pares the first 4 lines.
    /// The first line is the status, the second is the path, the third is the duration, and the fourth is the position.
    /// The rest of the lines are tags, and the player settings, so we'll send them to `TrackMetadata::parse`, to get the tags.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        Ok(Track::builder().status(
            TrackStatus::from_str(lines.next().ok_or("Missing status")?.split_once(' ')
                .ok_or("Unknown status")?.1)?
        )
            .path(lines.next().ok_or("Missing path")?.split_once(' ')
                .ok_or("Empty path")?.1.to_string())
            .duration(
                lines.next().ok_or("Missing duration")?.split_once(' ')
                    .ok_or("Empty duration")?.1.parse().map_err(|e: ParseIntError| e.to_string())?
            )
            .position(
                lines.next().ok_or("Missing position")?.split_once(' ')
                    .ok_or("Empty position")?.1.parse().map_err(|e: ParseIntError| e.to_string())?
            )
            .metadata(TrackMetadata::parse(lines))
            .build())
    }
}

impl TrackMetadata {
    /// Parse the tags from the rest of `cmus-remote -Q` output.
    /// This function will assume you processed the first 4 lines, and remove them from the iterator.
    ///
    /// and also assume the all tags is contained in the iterator.
    fn parse<'a>(mut lines: impl Iterator<Item=&'a str>) -> Self {
        let mut tags = HashMap::new();

        while let Some(line) = lines.next() {
            match line.trim().split_once(' ') {
                Some(("tag", rest)) => {
                    let Some((key, value)) = rest.split_once(' ') else {
                        continue; // Ignore lines that don't have a key and a value.
                    };
                    tags.insert(key.to_string(), value.to_string());
                }
                _ => break, // We've reached the end of the tags.
            }
        }

        TrackMetadata { tags }
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map(|s| s.as_str())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    const OUTPUT_WITH_ALL_TAGS: &str = include_str!("../../tests/samples/cmus-remote-output-with-all-tags.txt");

    const SOME_TAGS: &str = r#"tag artist Alex Goot
        tag album Alex Goot & Friends, Vol. 3
        tag title Photograph
        tag date 2014
        tag genre Pop
        tag discnumber 1
        tag tracknumber 8
        tag albumartist Alex Goot
        tag replaygain_track_gain -9.4 dB
        tag composer Chad Kroeger
        tag label mudhutdigital.com
        tag publisher mudhutdigital.com
        tag bpm 146
        set aaa_mode artist
        ..."#;

    #[test]
    fn test_create_track_from_str() {
        let track = Track::from_str(OUTPUT_WITH_ALL_TAGS);

        assert_matches!(track, Ok(_));

        let track = track.unwrap();

        assert_eq!(track.status, TrackStatus::Playing);
        assert_eq!(track.path, "/mnt/Data/Music/FLAC/Alex Goot/Alex Goot - Alex Goot & Friends, Vol. 3/08 - Photograph.mp3");
        assert_eq!(track.duration, 284);
        assert_eq!(track.position, 226);
        assert_eq!(track.metadata.tags.get("artist"), Some(&"Alex Goot".to_string()));
        assert_eq!(track.metadata.tags.get("album"), Some(&"Alex Goot & Friends, Vol. 3".to_string()));
        assert_eq!(track.metadata.tags.get("title"), Some(&"Photograph".to_string()));
        assert_eq!(track.metadata.tags.get("date"), Some(&"2014".to_string()));
        assert_eq!(track.metadata.tags.get("genre"), Some(&"Pop".to_string()));
        assert_eq!(track.metadata.tags.get("discnumber"), Some(&"1".to_string()));
        assert_eq!(track.metadata.tags.get("tracknumber"), Some(&"8".to_string()));
        assert_eq!(track.metadata.tags.get("albumartist"), Some(&"Alex Goot".to_string()));
        assert_eq!(track.metadata.tags.get("replaygain_track_gain"), Some(&"-9.4 dB".to_string()));
        assert_eq!(track.metadata.tags.get("composer"), Some(&"Chad Kroeger".to_string()));
        assert_eq!(track.metadata.tags.get("label"), Some(&"mudhutdigital.com".to_string()));
        assert_eq!(track.metadata.tags.get("publisher"), Some(&"mudhutdigital.com".to_string()));
        assert_eq!(track.metadata.tags.get("bpm"), Some(&"146".to_string()));
    }

    #[test]
    fn test_parse_metadata_from_the_string() {
        let metadata = TrackMetadata::parse(SOME_TAGS.lines());

        assert_eq!(metadata.tags.get("artist"), Some(&"Alex Goot".to_string()));
        assert_eq!(metadata.tags.get("album"), Some(&"Alex Goot & Friends, Vol. 3".to_string()));
        assert_eq!(metadata.tags.get("title"), Some(&"Photograph".to_string()));
        assert_eq!(metadata.tags.get("date"), Some(&"2014".to_string()));
        assert_eq!(metadata.tags.get("genre"), Some(&"Pop".to_string()));
        assert_eq!(metadata.tags.get("discnumber"), Some(&"1".to_string()));
        assert_eq!(metadata.tags.get("tracknumber"), Some(&"8".to_string()));
        assert_eq!(metadata.tags.get("albumartist"), Some(&"Alex Goot".to_string()));
        assert_eq!(metadata.tags.get("replaygain_track_gain"), Some(&"-9.4 dB".to_string()));
        assert_eq!(metadata.tags.get("composer"), Some(&"Chad Kroeger".to_string()));
        assert_eq!(metadata.tags.get("label"), Some(&"mudhutdigital.com".to_string()));
        assert_eq!(metadata.tags.get("publisher"), Some(&"mudhutdigital.com".to_string()));
        assert_eq!(metadata.tags.get("bpm"), Some(&"146".to_string()));
    }
}
